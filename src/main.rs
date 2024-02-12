use std::net::Ipv4Addr;
use std::str::FromStr;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_address = "175.45.183.112:1709";
    let listener = TcpListener::bind("127.0.0.1:2606").await?;

    loop {
        let client = listener.accept().await?;
        let server = TcpStream::connect(server_address).await?;
    }
}
