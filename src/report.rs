use std::fs::OpenOptions;
use std::io::prelude::*;
use std::net::SocketAddr;
use crate::socks::SocksVersion;

pub fn report_proxy(proxy: SocketAddr, socks_version: &SocksVersion) {
    println!("Proxy found (report function): {:?}", proxy);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("proxies.txt")
        .expect("Unable to open proxies file");

    //ProxyChains format
    let proxy_info = format!("{} {} {}\n", socks_version, proxy.ip(), proxy.port());

    file.write_all(proxy_info.as_bytes())
        .expect("Unable to write to proxies file");
}
