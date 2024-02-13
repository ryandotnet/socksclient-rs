use tokio::{io, join};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_addr = "175.45.183.112:1709";
    let listener = TcpListener::bind("127.0.0.1:2606").await?;

    loop {
        let client = listener.accept().await?;
        let server = TcpStream::connect(server_addr).await?;

        let (mut cread, mut cwrite) = client.into_split();
        let (mut sread, mut swrite) = server.into_split();

        let c2s = io::copy(&mut cread, &mut cwrite);
        let s2s = io::copy(&mut sread, &mut swrite);

    }
}
