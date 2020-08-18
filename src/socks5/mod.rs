use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, ErrorKind, Error, AsyncWriteExt};
use std::net::SocketAddr;
use crate::socks5::errors::{UnexpectedSocksVersionError, UnexpectedDataPackLengthError, UnsupportedIdentifierMethodError};

pub mod errors;
pub mod request;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IdentifierMethod {
    None,
    GSSAPI,
    Username,
    Unsupported,
}

pub async fn identifier_method_select(socket: &mut TcpStream, ad: &SocketAddr) -> std::io::Result<IdentifierMethod> {
    let mut buf = [0; 256];
    let n = socket.read(&mut buf[..]).await?;
    println!("Accepted {} bytes: {:?} from {:?}", n, &buf[..n], ad);

    if n <= 2 {
        return Err(Error::new(ErrorKind::UnexpectedEof, UnexpectedDataPackLengthError));
    }

    if buf[0] != 5 {
        //check version
        let e = UnexpectedSocksVersionError(buf[0]);
        return Err(Error::new(ErrorKind::InvalidData, e));
    }

    let n_methods = buf[1];

    for i in 0..n_methods {
        match buf[2 + i as usize] {
            0 => {
                return Ok(IdentifierMethod::None);
            }
            1 => {
                return Ok(IdentifierMethod::GSSAPI);
            }
            2 => {
                return Ok(IdentifierMethod::Username);
            }
            _ => {}
        }
    }

    Ok(IdentifierMethod::Unsupported)
}

pub async fn identifier_method_select_response(socket: &mut TcpStream, ad: &SocketAddr, method: IdentifierMethod) -> std::io::Result<()> {
    match method {
        IdentifierMethod::None => {
            socket.write_u8(5).await?;
            socket.write_u8(0).await?;
            socket.flush().await?;
            Ok(())
        }
        IdentifierMethod::GSSAPI => {
            socket.write_u8(5).await?;
            socket.write_u8(1).await?;
            socket.flush().await?;
            Ok(())
        }
        IdentifierMethod::Username => {
            socket.write_u8(5).await?;
            socket.write_u8(2).await?;
            socket.flush().await?;
            Ok(())
        }
        IdentifierMethod::Unsupported => {
            socket.flush().await?;
            Err(Error::new(ErrorKind::Other, UnsupportedIdentifierMethodError))
        }
    }
}

pub async fn sub_negotiation(socket: &mut TcpStream, ad: &SocketAddr, method: IdentifierMethod) -> std::io::Result<()> {
    match method {
        IdentifierMethod::None => {
            Ok(())
        }
        IdentifierMethod::GSSAPI => {
            Ok(())
        }
        IdentifierMethod::Username => {
            //todo username auth
            Ok(())
        }
        IdentifierMethod::Unsupported => {
            Ok(())
        }
    }
}