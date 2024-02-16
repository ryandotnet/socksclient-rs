use tokio::net::{TcpListener, TcpStream};
use tokio::{io, join};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2606").await?;

    loop {
        let (mut client, _) = listener.accept().await?;
        handle_client(client).await?;
    }
}

async fn handle_client(client: TcpStream) -> io::Result<()> {
    let server_addr = "175.45.183.112:1709";

    let server = TcpStream::connect(server_addr).await?;

    exchange_data(client, server);

    Ok(())
}

async fn auth_client() {}

async fn exchange_data(mut client: TcpStream, mut server: TcpStream) {
    io::copy_bidirectional(&mut client, &mut server);
    _ = tokio::spawn(async move { io::copy_bidirectional(&mut client, &mut server) })
}
