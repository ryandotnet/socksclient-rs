use std::error::Error;
use std::time::Duration;

use socks5::Socks5Error;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

mod socks5;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2606").await?;
    println!("Listening on: {}\n", listener.local_addr()?);

    loop {
        let (client, _) = listener.accept().await?;
        let _ = match handle_client(client).await {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("{}", err);
                Err(err)
            }
        };
    }
}

async fn handle_client(client: TcpStream) -> io::Result<()> {
    let server_addr = "100.45.183.112:1709";

    let server = TcpStream::connect(server_addr).await?;

    // socks5::Client::handshake(&mut server).await?;

    // exchange_data(client, server).await?;

    Ok(())
}

async fn test() {
    println!("Success!");
}

async fn exchange_data(mut client: TcpStream, mut server: TcpStream) -> io::Result<()> {
    tokio::spawn(async move { io::copy_bidirectional(&mut client, &mut server).await });
    Ok(())
}
