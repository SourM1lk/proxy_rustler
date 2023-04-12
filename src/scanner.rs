// src/scanner.rs

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{task, net::TcpStream};
use crate::config::ScannerConfig;
use crate::connection::connect_to_proxy;
use crate::socks::SocksVersion;

pub async fn scan_proxies(config: ScannerConfig) {
    let (start_ip, end_ip) = config.ip_range;
    let socks_version = match config.socks_version {
        4 => SocksVersion::V4,
        5 => SocksVersion::V5,
        _ => unreachable!(),
    };

    let mut tasks = Vec::new();

    for ip in ip_range(start_ip, end_ip) {
        for port in 1..=65535 {
            let proxy_addr = SocketAddr::new(ip, port);
            let target_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53); // For example, we can use Google DNS as the target

            let socks_version = socks_version.clone();
            let task = task::spawn(async move {
                match TcpStream::connect(proxy_addr).await {
                    Ok(_) => {
                        if let Ok(granted) = connect_to_proxy(proxy_addr, target_addr, socks_version).await {
                            if granted {
                                println!("Proxy found: {:?}", proxy_addr);
                            }
                        }
                    }
                    Err(_) => {}
                }
            });

            tasks.push(task);
        }
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }
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
