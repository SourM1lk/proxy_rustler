use std::io::{Cursor, Read, Write};
use std::net::IpAddr;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;

#[derive(Debug, Clone)]
pub enum SocksVersion {
    V4,
    V5,
}

// Display for file save. Will need to modify if I want different formatting
impl fmt::Display for SocksVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SocksVersion::V4 => write!(f, "SOCKS4"),
            SocksVersion::V5 => write!(f, "SOCKS5"),
        }
    }
}

pub struct SocksRequest {
    pub version: SocksVersion,
    pub ip: IpAddr,
    pub port: u16,
}

pub enum SocksResponse {
    V4(u8),
    V5(u8, u8),
}

impl SocksRequest {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        match self.version {
            SocksVersion::V4 => {
                buf.write_u8(4).unwrap(); // SOCKS version 4
                buf.write_u8(1).unwrap(); // CONNECT command
                buf.write_u16::<BigEndian>(self.port).unwrap(); // Destination port

                match self.ip {
                    IpAddr::V4(ip) => buf.write_all(&ip.octets()).unwrap(),
                    _ => panic!("SOCKS4 supports only IPv4 addresses."),
                };

                buf.write_u8(0).unwrap(); // Null byte as the user ID terminator
            }
            SocksVersion::V5 => {
                buf.write_u8(5).unwrap(); // SOCKS version 5
                buf.write_u8(1).unwrap(); // CONNECT command
                buf.write_u8(0).unwrap(); // Reserved byte

                match self.ip {
                    IpAddr::V4(ip) => {
                        buf.write_u8(1).unwrap(); // Address type: IPv4
                        buf.write_all(&ip.octets()).unwrap();
                    }
                    IpAddr::V6(ip) => {
                        buf.write_u8(4).unwrap(); // Address type: IPv6
                        buf.write_all(&ip.octets()).unwrap();
                    }
                };

                buf.write_u16::<BigEndian>(self.port).unwrap(); // Destination port
            }
        }

        buf
    }
}

impl SocksResponse {
    pub fn from_bytes(version: &SocksVersion, bytes: &[u8]) -> Result<SocksResponse, Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(bytes);
    
        match version {
            SocksVersion::V4 => {
                cursor.read_u8()?; // Read and discard the null byte
                let status = cursor.read_u8()?;
                Ok(SocksResponse::V4(status))
            }
            SocksVersion::V5 => {
                cursor.read_u8()?; // Read and discard the version byte
                let status = cursor.read_u8()?;
                let reserved = cursor.read_u8()?;
                let addr_type = cursor.read_u8()?;
    
                match addr_type {
                    1 => {
                        let _ip_v4 = cursor.read_u32::<BigEndian>()?;
                    }
                    3 => {
                        let domain_len = cursor.read_u8()?;
                        let mut domain = vec![0; domain_len as usize];
                        cursor.read_exact(&mut domain)?;
                    }
                    4 => {
                        let _ip_v6 = cursor.read_u128::<BigEndian>()?;
                    }
                    _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid address type in the SOCKS5 response."))),
                };
    
                let _port = cursor.read_u16::<BigEndian>()?;
                Ok(SocksResponse::V5(reserved, status))
            }
        }
    }    
}
