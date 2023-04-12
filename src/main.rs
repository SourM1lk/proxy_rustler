// src/main.rs

use structopt::StructOpt;
use tokio::runtime;

mod config;
mod socks;
mod connection;
mod scanner;
mod report;

use crate::config::{CliOptions, ScannerConfig};

fn main() {
    let options = CliOptions::from_args();
    let config = ScannerConfig::from_options(options).expect("Failed to parse command-line options");

    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(scanner::scan_proxies(config));
}
