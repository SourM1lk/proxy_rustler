mod config;
mod socks;
mod connection;
mod scanner;
mod report;

use config::CliOptions;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let options = CliOptions::from_args();
    let config = config::ScannerConfig::from_options(options).expect("Invalid options");

    // Start scanning
}
