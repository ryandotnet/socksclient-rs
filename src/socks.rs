use std::net::Ipv4Addr;

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct Client;

impl Client {
    pub async fn handshake(mut server: TcpStream) -> io::Result<()> {
        let greeting = vec![5, 1, 2];
        server.write_all(&greeting).await?;

        let mut response = [0, 2];
        server.read_exact(&mut response).await?;

        match response[1] {
            2 => {
                Client::authenticate(server).await?;
            }
            _ => println!("Handshake Failed!"),
        }

        Ok(())
    }

    async fn authenticate(mut server: TcpStream) -> io::Result<()> {
        let user = String::from("root");
        let pass = String::from("j3hxgvbdo");

        let mut request = vec![1];
        request.push(user.len() as u8);
        request.extend_from_slice(user.as_bytes());
        request.push(pass.len() as u8);
        request.extend_from_slice(pass.as_bytes());

        server.write_all(&request).await?;

        let mut response = [0, 2];
        server.read_exact(&mut response).await?;

        if response[0] != 0 {
            println!("Authentication Failed!");
        }

        Ok(())
    }

    async fn request(mut server: TcpStream) -> io::Result<()> {
        let ip = Ipv4Addr::new(103, 100, 36, 63);
        let port: u16 = 1709;

        let mut request = vec![5, 1, 0, 1];
        request.extend_from_slice(&ip.octets());
        request.extend_from_slice(&port.to_be_bytes());

        server.write_all(&request).await?;

        let mut response = vec![0, 2];
        server.read_exact(&mut response).await?;

        if response[1] != 0 {
            println!("Remote connection request failed!");
        }

        Ok(())
    }
}
