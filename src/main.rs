use anyhow::{anyhow, Result};
use std::time::Duration;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

mod socks5;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2606").await?;

    println!("Listening on: {}\n", listener.local_addr()?);

    loop {
        let (client_stream, _) = listener.accept().await?;
        let _ = match handle_client(client_stream).await {
            Ok(()) => Ok(()),
            Err(error) => {
                println!("Error: {}", error);
                Err(error)
            }
        };
    }
}

async fn handle_client(client_stream: TcpStream) -> Result<()> {
    let server_addr = "50.50.50.50:5050";
    let mut server_stream: TcpStream =
        match tokio::time::timeout(Duration::from_millis(250), TcpStream::connect(server_addr))
            .await
        {
            Ok(stream) => stream?,
            Err(_) => {
                return Err(anyhow!(
                    "Failed to connect to server. Connection timed out."
                ))
            }
        };

    socks5::Client::handshake(&mut server_stream).await?;

    exchange_data(client_stream, server_stream).await?;

    Ok(())
}

async fn exchange_data(mut client: TcpStream, mut server: TcpStream) -> Result<()> {
    tokio::spawn(async move { io::copy_bidirectional(&mut client, &mut server).await });
    Ok(())
}
