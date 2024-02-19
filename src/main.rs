use socks5::Socks5Error;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

mod socks5;

#[tokio::main]
async fn main() -> Result<(), Socks5Error> {
    let listener = TcpListener::bind("127.0.0.1:2606").await?;
    println!("Listening on: {}\n", listener.local_addr()?);

    loop {
        let (client, _) = listener.accept().await?;
        handle_client(client).await?;
    }
}

async fn handle_client(client: TcpStream) -> Result<(), Socks5Error> {
    let svr_addr = "175.45.183.112:1709";

    let mut server = TcpStream::connect(svr_addr).await?;
    server.set_nodelay(true)?;

    socks5::Client::handshake(&mut server).await?;

    exchange_data(client, server).await?;

    Ok(())
}

async fn exchange_data(mut client: TcpStream, mut server: TcpStream) -> io::Result<()> {
    tokio::spawn(async move { io::copy_bidirectional(&mut client, &mut server).await });
    Ok(())
}
