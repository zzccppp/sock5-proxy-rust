use tokio::net::{TcpListener, TcpStream};
use std::net::{SocketAddr, SocketAddrV6, Ipv6Addr};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};
use crate::socks5::{identifier_method_select, identifier_method_select_response, sub_negotiation};
use crate::socks5::request::request_handler;

pub mod socks5;

#[tokio::main]
pub async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:8883").await.unwrap();
    loop {
        let (socket, x) = listener.accept().await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            match conn_handler(socket, x, local_addr).await {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        });
    }
}

async fn conn_handler(mut socket: TcpStream, ad: SocketAddr, local_addr: SocketAddr) -> std::io::Result<()> {
    let method = identifier_method_select(&mut socket, &ad).await?;
    identifier_method_select_response(&mut socket, &ad, method).await?;
    sub_negotiation(&mut socket, &ad, method).await?;
    request_handler(socket, &ad, &local_addr).await?;
    Ok(())
}
