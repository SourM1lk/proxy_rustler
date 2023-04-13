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

/*NOTES:
SOCKS4 Request Message:
Byte Stream
0x04 0x01 0x00 0x50 0xC0 0xA8 0x01 0x64 0x75 0x73 0x65 0x72 0x00

0x04010050C0A801647573657200

0x04 - The SOCKS version number (in this case, 4)
0x01 - The command code (in this case, 1 for CONNECT)
0x00 0x50 - The destination port (in this case, 80)
0xC0 0xA8 0x01 0x64 - The destination IP address (in this case, 192.168.1.100)
0x75 0x73 0x65 0x72 0x00 - The user ID (in this case, "user" terminated by a null byte)

+----+----+----+------+----------+----------+----------+
| VN | CD | DSTPORT |      DSTIP        | USERID | NULL|
+----+----+----+------+----------+----------+----------+
| 04 | 01 |   80    |    192.168.1.100  |  user  |   0 |
+----+----+----+------+----------+----------+----------+

SOCKS5 Request Message:
Byte Stream
0x05 0x01 0x00 0x01 0xC0 0xA8 0x01 0x64 0x00 0x50

0x0501000101C0A801640050

0x05 - The SOCKS version number (in this case, 5)
0x01 - The command code (in this case, 1 for CONNECT)
0x00 - A reserved byte
0x01 - The address type (in this case, 1 for an IPv4 address)
0xC0 0xA8 0x01 0x64 - The destination IP address (in this case, 192.168.1.100)
0x00 0x50 - The destination port (in this case, 80)

+----+-----+-------+------+----------+----------+
|VER | CMD |  RSV  | ATYP | DST.ADDR | DST.PORT |
+----+-----+-------+------+----------+----------+
| 05 | 01  |  00   |  01  |  192.168.1.100 | 80 |
+----+-----+-------+------+----------+----------+
 */