use tokio::net::TcpStream;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, Ipv6Addr, IpAddr};
use tokio::io::{AsyncReadExt, Error, ErrorKind, AsyncWriteExt};
use crate::socks5::errors::{UnexpectedSocksVersionError, UnexpectedDataError};

fn get_address(buf: &[u8]) -> std::io::Result<String> {
    let atyp = buf[3];
    match atyp {
        1 => {
            //Ipv4
            let port = u16::from_be_bytes([buf[8], buf[9]]);
            let re = format!("{}.{}.{}.{}:{}", buf[4], buf[5], buf[6], buf[7], port);
            Ok(re)
        }
        3 => {
            //Domain
            let len = buf[4];
            let mut domain = Vec::<u8>::new();
            for i in 5..(5 + len) {
                domain.push(buf[i as usize]);
            }
            let mut re = String::from_utf8(domain).unwrap_or("0.0.0.0".to_string());
            let port = u16::from_be_bytes([buf[5 + len as usize], buf[5 + len as usize + 1]]);
            re.push_str(&format!(":{}", port));
            Ok(re)
        }
        4 => {
            //Ipv6
            let port = u16::from_be_bytes([buf[20], buf[21]]);
            let addr = SocketAddrV6::new(Ipv6Addr::from([
                buf[4], buf[5], buf[6], buf[7],
                buf[8], buf[9], buf[10], buf[11],
                buf[12], buf[13], buf[14], buf[15],
                buf[16], buf[17], buf[18], buf[19],
            ]), port, 0, 0);
            let re = addr.to_string();
            Ok(re)
        }
        _ => {
            return Err(Error::new(ErrorKind::InvalidData, UnexpectedDataError));
        }
    }
}

pub async fn request_handler(mut socket: TcpStream, ad: &SocketAddr, local_addr: &SocketAddr) -> std::io::Result<()> {
    let mut buf = [0; 512];
    let n = socket.read(&mut buf[..]).await?;
    println!("Accepted {} bytes: {:?} from {:?}", n, &buf[..n], ad);

    // VER CMD RSV ATYP DST.ADDR DST.PORT
    // 0   1   2   3    4        -1 -2

    if buf[0] != 5 {
        //check version
        let e = UnexpectedSocksVersionError(buf[0]);
        return Err(Error::new(ErrorKind::InvalidData, e));
    }

    match buf[1] {
        1 => {
            //CONNECT REQUEST
            let addr_string = get_address(&buf)?;
            println!("{}", addr_string);
            let mut target_stream = match TcpStream::connect(addr_string).await {
                Ok(e) => {
                    e
                }
                Err(e) => {
                    socket.write(&[5, 1, 0, 1, 0, 0, 0, 0, 0, 0]).await?;
                    return Err(e);
                }
            };
            socket.write(&[5, 0, 0]).await?;//VER SUCCESS RSV
            let ip = local_addr.ip();
            match ip {
                IpAddr::V4(v4) => {
                    socket.write_u8(1).await?;
                    socket.write(&v4.octets()).await?;
                }
                IpAddr::V6(v6) => {
                    socket.write_u8(4).await?;
                    socket.write(&v6.octets()).await?;
                }
            }
            socket.write_u16(local_addr.port()).await?;
            socket.flush().await?;

            //Start forwarding data

            println!("--start--");

            let (mut local_read, mut local_write) = socket.into_split();
            let (mut remote_read, mut remote_write) = target_stream.into_split();

            let t1 = tokio::spawn(async move {
                let mut buf1 = [0u8; 512];
                let mut n = local_read.read(&mut buf1).await.unwrap();
                while n != 0 {
                    remote_write.write(&mut buf1[0..n]).await.unwrap();
                    n = local_read.read(&mut buf1).await.unwrap();
                }
            });

            let t2 = tokio::spawn(async move {
                let mut buf1 = [0u8; 512];
                let mut n = remote_read.read(&mut buf1).await.unwrap();
                while n != 0 {
                    local_write.write(&mut buf1[0..n]).await.unwrap();
                    n = remote_read.read(&mut buf1).await.unwrap();
                }
            });

            println!("--end--");
        }
        2 => {
            //todo bind
        }
        3 => {
            //todo UDP ASSOCIATE
        }
        _ => {
            return Err(Error::new(ErrorKind::InvalidData, UnexpectedDataError));
        }
    }

    Ok(())
}