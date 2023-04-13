use std::net::IpAddr;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Version",
    about = "A Rust-Based SOCKS Proxy Scanner",
)]
pub struct CliOptions {
    pub ip_range: String,
    #[structopt(long = "socks", short = "s", default_value = "4", use_delimiter = true)]
    pub socks_versions: Vec<u8>,    
    #[structopt(long = "connection_limit", short = "c", default_value = "1000")]
    pub connection_limit: usize,
    #[structopt(long = "port", short = "p", default_value = "1-65535", use_delimiter = true)]
    pub port_range: Vec<u16>,
}

pub struct ScannerConfig {
    pub ip_range: (IpAddr, IpAddr),
    pub socks_versions: Vec<u8>,
    pub connection_limit: usize,
    pub port_range: Vec<u16>,
}

impl ScannerConfig {
    pub fn from_options(options: CliOptions) -> Result<ScannerConfig, &'static str> {
        // Parse the IP range
        let ip_range = parse_ip_range(&options.ip_range)?;

        // Validate the SOCKS version
        for &version in &options.socks_versions {
            if version != 4 && version != 5 {
                return Err("Invalid SOCKS version. Please specify either 4 or 5.");
            }
        }

        Ok(ScannerConfig {
            ip_range,
            socks_versions: options.socks_versions,
            connection_limit: options.connection_limit,
            port_range: options.port_range
        })
    }
}

fn parse_ip_range(ip_range_str: &str) -> Result<(IpAddr, IpAddr), &'static str> {
    let ips: Vec<&str> = ip_range_str.split('-').collect();
    if ips.len() != 2 {
        return Err("Invalid IP range format. Please use the format: START_IP-END_IP.");
    }

    let start_ip = IpAddr::from_str(ips[0]).map_err(|_| "Invalid start IP address.")?;
    let end_ip = IpAddr::from_str(ips[1]).map_err(|_| "Invalid end IP address.")?;

    Ok((start_ip, end_ip))
}
