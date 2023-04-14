use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{task, net::TcpStream};
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::Semaphore;
use crate::config::{ScannerConfig, Port};
use crate::connection::connect_to_proxy;
use crate::socks::SocksVersion;
use crate::report::report_proxy;

pub async fn scan_proxies(config: ScannerConfig) -> Vec<SocketAddr> {
    // Create a semaphore to limit concurrent connections
    // Different OSs have different limits
    let semaphore = Arc::new(Semaphore::new(config.connection_limit));
    let (start_ip, end_ip) = config.ip_range;
    let mut valid_proxies = Vec::new();
    let mut tasks = Vec::new();

    // Calculate the total number of items IPs * Ports to be scanned
    let total_ports: u64 = config.ports.iter().map(|port_config| {
        match port_config {
            Port::Single(_) => 1,
            Port::Range(port_range) => (port_range.end - port_range.start + 1) as u64,
        }
    }).sum();
    
    let total_ips = match (start_ip, end_ip) {
        (IpAddr::V4(start_ipv4), IpAddr::V4(end_ipv4)) => {
            let num_ips = u32::from(end_ipv4) - u32::from(start_ipv4) + 1;
            (num_ips as u64) * total_ports
        },
        _ => panic!("Both start and end IP addresses must be IPv4"),
    };

    // Initialize a progress bar
    let pb = ProgressBar::new(total_ips);
    let progress_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7}")
        .expect("Failed to create progress bar template")
        .progress_chars("=>-");
    pb.set_style(progress_style);

    // Iterate through IPs in the range
    for ip in ip_range(start_ip, end_ip) {
        // Iterate through all possible ports
        for port_config in &config.ports {
            let ports = match port_config {
                Port::Single(port) => vec![*port],
                Port::Range(port_range) => (port_range.start..=port_range.end).collect::<Vec<u16>>(),
            };
        // Iterate through the ports
        for port in ports {
            let proxy_addr = SocketAddr::new(ip, port);
            let target_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);
            
            // Clone SOCKS versions, progress bar, and semaphore
            let socks_versions = config.socks_versions.clone();
            let pb_clone = pb.clone();
            let semaphore_clone = Arc::clone(&semaphore);

            // Spawn a task to scan the current IP and port
            let task = task::spawn(async move {
                // Acquire a permit from the semaphore
                let _permit = semaphore_clone.acquire().await.unwrap();

                 // Iterate through the selected SOCKS versions
                for &socks_version in &socks_versions {
                    let socks_version_enum = match socks_version {
                        4 => SocksVersion::V4,
                        5 => SocksVersion::V5,
                        _ => unreachable!(),
                    };

                    // Check if the proxy works for the current SOCKS version
                    //TODO: ADD duration timeout flag
                    match timeout(Duration::from_secs(config.timeout), TcpStream::connect(proxy_addr)).await {
                        Ok(_) => {
                            if let Ok(granted) = connect_to_proxy(proxy_addr, target_addr, socks_version_enum.clone()).await {
                                if granted {
                                    println!("Found valid proxy: {:?}", proxy_addr);
                                    report_proxy(proxy_addr, &socks_version_enum);
                                    println!("Proxy reported: {:?}", proxy_addr);
                                    return Some(proxy_addr);
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
                // Update the progress bar
                pb_clone.inc(1);
                None
            });
            tasks.push(task);
        }
    }
}

    // Wait for all tasks to complete and collect valid proxies
    for task in tasks {
        if let Some(proxy) = task.await.unwrap() {
            valid_proxies.push(proxy);
        }
    }
    // Finish the progress bar
    pb.finish();
    valid_proxies
}

fn ip_range(start_ip: IpAddr, end_ip: IpAddr) -> impl Iterator<Item = IpAddr> {
    let start = match start_ip {
        IpAddr::V4(v4) => u32::from(v4),
        _ => panic!("Start IP must be IPv4"),
    };
    let end = match end_ip {
        IpAddr::V4(v4) => u32::from(v4),
        _ => panic!("End IP must be IPv4"),
    };

    (start..=end).map(|ip| IpAddr::V4(Ipv4Addr::from(ip)))
}
