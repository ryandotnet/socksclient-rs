use std::{fmt, net::Ipv4Addr};

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

enum Socks5Error {
    HandshakeFailed,
    AuthenticationFailed,
    RequestFailed,
    IoError(std::io::ErrorKind),
}

impl From<std::io::Error> for Socks5Error {
    fn from(err: std::io::Error) -> Self {
        Socks5Error::IoError(err.kind())
    }
}

impl fmt::Display for Socks5Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Socks5Error::HandshakeFailed => write!(f, "Handshake failed."),
            Socks5Error::AuthenticationFailed => write!(f, "Authentication failed."),
            Socks5Error::RequestFailed => write!(f, "Request failed."),
            Self::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

pub struct Client;

impl Client {
    pub async fn handshake(server: &mut TcpStream) -> Result<(), Socks5Error> {
        let greeting = vec![5, 1, 2];
        server.write_all(&greeting).await?;

        let mut response = [0, 2];
        server.read_exact(&mut response).await?;

        match response[1] {
            2 => Client::authenticate(server).await?,
            _ => return Err(Socks5Error::HandshakeFailed),
        }

        Ok(())
    }

    async fn authenticate(server: &mut TcpStream) -> Result<(), Socks5Error> {
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

        match response[1] {
            0 => Client::request(server).await?,
            _ => return Err(Socks5Error::AuthenticationFailed),
        }

        Ok(())
    }

    async fn request(server: &mut TcpStream) -> Result<(), Socks5Error> {
        let ip = Ipv4Addr::new(103, 100, 36, 63);
        let port: u16 = 1709;

        let mut request = vec![5, 1, 0, 1];
        request.extend_from_slice(&ip.octets());
        request.extend_from_slice(&port.to_be_bytes());
        server.write_all(&request).await?;

        let mut response = vec![0, 2];
        server.read_exact(&mut response).await?;

        match response[1] {
            0 => Ok(()),
            _ => return Err(Socks5Error::RequestFailed),
        }
    }
}
