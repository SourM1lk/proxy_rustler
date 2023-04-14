use std::io;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::socks::{SocksRequest, SocksResponse, SocksVersion};

pub async fn connect_to_proxy(proxy_addr: SocketAddr, target_addr: SocketAddr, version: SocksVersion) -> Result<bool, io::Error> {
    let mut stream = TcpStream::connect(proxy_addr).await?;

    let ip = target_addr.ip();
    let port = target_addr.port();

    // Create and send the SOCKS request
    let socks_request = SocksRequest {
        version: version.clone(),
        ip,
        port,
    };

    let request_bytes = socks_request.to_bytes();
    println!("Request bytes: {:?} from {}", request_bytes, proxy_addr);
    stream.write_all(&request_bytes).await?;


    // Receive and parse the SOCKS response
    let mut response_buf = vec![0; 512]; // A buffer to store the response
    let read_bytes = stream.read(&mut response_buf).await?;
    response_buf.truncate(read_bytes);

    println!("Response bytes: {:?}", response_buf);

    let socks_response = SocksResponse::from_bytes(&version, &response_buf).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid SOCKS response"))?;

    // Check if the proxy server granted the request
    println!("socks_response: {:?}", socks_response);
    match socks_response {
        SocksResponse::V4(status) => {
            println!("Enter Socks4 Response");
            Ok(status == 0x5A)
        }
        SocksResponse::V5(status, _) => {
            println!("Enter Socks5 Response");
            Ok(status == 0x00)
        }
    }
}
