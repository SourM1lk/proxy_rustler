use std::net::IpAddr;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct CliOptions {
    pub ip_range: String,
    #[structopt(long = "socks", short = "s", default_value = "4")]
    pub socks_version: u8,
}

pub struct ScannerConfig {
    pub ip_range: (IpAddr, IpAddr),
    pub socks_version: u8,
}

impl ScannerConfig {
    pub fn from_options(options: CliOptions) -> Result<ScannerConfig, &'static str> {
        // Parse the IP range
        let ip_range = parse_ip_range(&options.ip_range)?;

        // Validate the SOCKS version
        if options.socks_version != 4 && options.socks_version != 5 {
            return Err("Invalid SOCKS version. Please specify either 4 or 5.");
        }

        Ok(ScannerConfig {
            ip_range,
            socks_version: options.socks_version,
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
