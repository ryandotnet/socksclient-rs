use std::error::Error;
use std::io::ErrorKind;
use std::time::Duration;

use socks5::Socks5Error;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

mod socks5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:2606").await?;
    println!("Listening on: {}\n", listener.local_addr()?);

    loop {
        let (client_stream, _) = listener.accept().await?;
        let _ = match handle_client(client_stream).await {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.downcast_ref::<Socks5Error>().is_some() {
                    println!("Socks Error: {}", err);
                }

                if let Some(io_error) = err.downcast_ref::<std::io::Error>() {
                    match io_error.kind() {
                        ErrorKind::TimedOut => {
                            println!("Failed to connect to server. Connection timed out.")
                        }
                        _ => println!("Failed to connect to server. IO Error: {}", io_error),
                    }
                }

                Err(err)
            }
        };
    }
}

async fn handle_client(client_stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let server_addr = "175.45.183.112:1709";

    let mut server_stream = TcpStream::connect(server_addr).await?;

    socks5::Client::handshake(&mut server_stream).await?;

    exchange_data(client_stream, server_stream).await?;

    Ok(())
}

async fn exchange_data(mut client: TcpStream, mut server: TcpStream) -> io::Result<()> {
    tokio::spawn(async move { io::copy_bidirectional(&mut client, &mut server).await });
    Ok(())
}
