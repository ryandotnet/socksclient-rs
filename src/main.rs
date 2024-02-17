use tokio::io;
use tokio::net::{TcpListener, TcpStream};

mod socks;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2606").await?;
    println!("Listening on: {}\n", listener.local_addr()?);

    loop {
        let (client, _) = listener.accept().await?;
        handle_client(client).await?;
    }
}

async fn handle_client(client: TcpStream) -> io::Result<()> {
    let svr_addr = "175.45.183.112:1709";

    let server = TcpStream::connect(svr_addr).await?;
    server.set_nodelay(true)?;

    println!("New connection opened.");

    exchange_data(client, server).await?;

    Ok(())
}

async fn exchange_data(mut client: TcpStream, mut server: TcpStream) -> io::Result<()> {
    tokio::spawn(async move { io::copy_bidirectional(&mut client, &mut server).await });
    Ok(())
}
