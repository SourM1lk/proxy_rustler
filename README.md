# Proxy Rustler
Proxy Rustler is a fast, concurrent proxy scanner written in Rust. It scans for SOCKS4 and SOCKS5 proxies within the specified IP range and port(s). The scanner utilizes async programming to achieve high performance and concurrency.

## Features
```
Supports SOCKS4 and SOCKS5 protocols
Scans a specified IP range and port(s)
Concurrent scanning for faster results
Progress bar to track the scanning process
Configurable timeout and connection limit
```

## Installation
Ensure you have Rust installed. If you don't have it yet, you can get it from rust-lang.org.

Clone the repository:
```
git clone https://github.com/username/proxy_rustler.git
cd proxy_rustler
```
Build the project:
```
cargo build --release
```

## Usage
```
./proxy_rustler START_IP-END_IP --port PORT(S)
```
Example:
```
./proxy_rustler 74.119.100.0-74.119.150.255 --port 4145 --socks 4
./proxy_rustler 192.168.0.0-192.168.255.255 --port 1080,4145 --socks 5
./proxy_rustler 10.0.0.0-10.255.255.255 --port 1000-2000 --socks 4,5
```

## Help
```
    _  (`-')   (`-')            (`-')                      (`-')             (`-').->(`-')              (`-')  _   (`-')  
    \-.(OO )<-.(OO )      .->   (OO )_.->     .->       <-.(OO )      .->    ( OO)_  ( OO).->    <-.    ( OO).-/<-.(OO )  
    _.'    \,------,)(`-')----. (_| \_)--.,--.'  ,-.    ,------,),--.(,--.  (_)--\_) /    '._  ,--. )  (,------.,------,) 
   (_...--''|   /`. '( OO).-.  '\  `.'  /(`-')'.'  /    |   /`. '|  | |(`-')/    _ / |'--...__)|  (`-') |  .---'|   /`. ' 
   |  |_.' ||  |_.' |( _) | |  | \    .')(OO \    /     |  |_.' ||  | |(OO )\_..`--. `--.  .--'|  |OO )(|  '--. |  |_.' | 
   |  .___.'|  .   .' \|  |)|  | .'    \  |  /   /)     |  .   .'|  | | |  \.-._)   \   |  |  (|  '__ | |  .--' |  .   .' 
   |  |     |  |\  \   '  '-'  '/  .'.  \ `-/   /`      |  |\  \ \  '-'(_ .'\       /   |  |   |     |' |  `---.|  |\  \  
   `--'     `--' '--'   `-----'`--'   '--'  `--'        `--' '--' `-----'    `-----'    `--'   `-----'  `------'`--' '--'    
                                                                                 
Version 0.1.0
A Rust-Based SOCKS Proxy Scanner

USAGE:
    proxy_rustler [OPTIONS] <ip-range>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --connection_limit <connection-limit>     [default: 1000]
    -p, --port <ports>...                         [default: 1-65535]
    -s, --socks <socks-versions>...               [default: 4]
    -t, --timeout <timeout>                       [default: 5]

ARGS:
    <ip-range>    
```

## License
Proxy Rustler is licensed under the MIT license. See [LICENSE](LICENSE) for more information.