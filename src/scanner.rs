use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{task, net::TcpStream};
use tokio::time::{timeout, Duration};
use crate::config::ScannerConfig;
use crate::connection::connect_to_proxy;
use crate::socks::SocksVersion;
use crate::report::report_proxy;



pub async fn scan_proxies(config: ScannerConfig) -> Vec<SocketAddr>  {
    let (start_ip, end_ip) = config.ip_range;
    let socks_version = match config.socks_version {
        4 => SocksVersion::V4,
        5 => SocksVersion::V5,
        _ => unreachable!(),
    };

    let mut valid_proxies = Vec::new();
    let mut tasks = Vec::new();

    for ip in ip_range(start_ip, end_ip) {
        for port in 1..=65535 {
            let proxy_addr = SocketAddr::new(ip, port);
            let target_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);

            let socks_version = socks_version.clone();
            let task = task::spawn(async move {
                match timeout(Duration::from_secs(4), TcpStream::connect(proxy_addr)).await {
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
