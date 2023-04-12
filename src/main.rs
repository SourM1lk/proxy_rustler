use structopt::StructOpt;
use tokio::runtime;
use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod config;
mod socks;
mod connection;
mod scanner;
mod report;

use crate::config::{CliOptions, ScannerConfig};

fn main() {
    display_welcome_message();

    let options = CliOptions::from_args();
    let config = ScannerConfig::from_options(options).expect("Failed to parse command-line options");

    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(scanner::scan_proxies(config));
}

fn display_welcome_message() {
    let logo = r#"

    _  (`-')   (`-')            (`-')                      (`-')             (`-').->(`-')              (`-')  _   (`-')  
    \-.(OO )<-.(OO )      .->   (OO )_.->     .->       <-.(OO )      .->    ( OO)_  ( OO).->    <-.    ( OO).-/<-.(OO )  
    _.'    \,------,)(`-')----. (_| \_)--.,--.'  ,-.    ,------,),--.(,--.  (_)--\_) /    '._  ,--. )  (,------.,------,) 
   (_...--''|   /`. '( OO).-.  '\  `.'  /(`-')'.'  /    |   /`. '|  | |(`-')/    _ / |'--...__)|  (`-') |  .---'|   /`. ' 
   |  |_.' ||  |_.' |( _) | |  | \    .')(OO \    /     |  |_.' ||  | |(OO )\_..`--. `--.  .--'|  |OO )(|  '--. |  |_.' | 
   |  .___.'|  .   .' \|  |)|  | .'    \  |  /   /)     |  .   .'|  | | |  \.-._)   \   |  |  (|  '__ | |  .--' |  .   .' 
   |  |     |  |\  \   '  '-'  '/  .'.  \ `-/   /`      |  |\  \ \  '-'(_ .'\       /   |  |   |     |' |  `---.|  |\  \  
   `--'     `--' '--'   `-----'`--'   '--'  `--'        `--' '--' `-----'    `-----'    `--'   `-----'  `------'`--' '--'    
                                                                                 "#;

    let mut colors = vec![
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white",
    ];
    let mut rng = thread_rng();
    colors.shuffle(&mut rng);
    let mut index = 0;
    for line in logo.lines() {
        println!("{}", line.color(colors[index % colors.len()]).bold());
        index += 1;
    }
}