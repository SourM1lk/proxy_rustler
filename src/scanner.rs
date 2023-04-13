use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{task, net::TcpStream};
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use crate::config::ScannerConfig;
use crate::connection::connect_to_proxy;
use crate::socks::SocksVersion;
use crate::report::report_proxy;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::Semaphore;


pub async fn scan_proxies(config: ScannerConfig) -> Vec<SocketAddr> {
    //TODO: make this a command line argument
    let connection_limit = 1000; 
    let semaphore = Arc::new(Semaphore::new(connection_limit));

    let (start_ip, end_ip) = config.ip_range;
    let socks_version = match config.socks_version {
        4 => SocksVersion::V4,
        5 => SocksVersion::V5,
        _ => unreachable!(),
    };

    let mut valid_proxies = Vec::new();
    let mut tasks = Vec::new();

    let total_ips = match (start_ip, end_ip) {
        (IpAddr::V4(start_ipv4), IpAddr::V4(end_ipv4)) => ((u32::from(end_ipv4) - u32::from(start_ipv4) + 1) * 65535) as u64,
        _ => panic!("Both start and end IP addresses must be IPv4"),
    };

    let pb = ProgressBar::new(total_ips);

    let progress_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
        .expect("Failed to create progress bar template")
        .progress_chars("=>-");

    pb.set_style(progress_style);



    for ip in ip_range(start_ip, end_ip) {
        for port in 1..=65535 {
            let proxy_addr = SocketAddr::new(ip, port);
            let target_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);

            let socks_version = socks_version.clone();
            let pb_clone = pb.clone();
            let semaphore_clone = Arc::clone(&semaphore);
            let task = task::spawn(async move {
                let _permit = semaphore_clone.acquire().await.unwrap();

                match timeout(Duration::from_secs(5), TcpStream::connect(proxy_addr)).await {
                    Ok(_) => {
                        if let Ok(granted) = connect_to_proxy(proxy_addr, target_addr, socks_version.clone()).await {
                            if granted {
                                report_proxy(proxy_addr, &socks_version);
                                return Some(proxy_addr);
                            }
                        }
                    }
                    Err(_) => {} 
                }
                if proxy_addr.port() == 4145 {
                    println!("Scanned: {:?}", proxy_addr);
                }

                pb_clone.inc(1);
                None
            });
            tasks.push(task);
        }
    }

    // Wait for all tasks to complete and collect valid proxies
    for task in tasks {
        if let Some(proxy) = task.await.unwrap() {
            valid_proxies.push(proxy);
        }
    }

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
