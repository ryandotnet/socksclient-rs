use anyhow::{anyhow, Result};
use std::net::Ipv4Addr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct Client;

impl Client {
    pub async fn handshake(server: &mut TcpStream) -> Result<()> {
        let greeting = vec![5, 1, 2];
        server.write_all(&greeting).await?;

        let mut response = [0; 2];
        server.read_exact(&mut response).await?;

        match response[1] {
            2 => Client::authenticate(server).await?,
            _ => return Err(anyhow!("Handshake failed.")),
        };

        Ok(())
    }

    async fn authenticate(server: &mut TcpStream) -> Result<()> {
        let user = String::from("user");
        let pass = String::from("pass");

        let mut request = vec![1];
        request.push(user.len() as u8);
        request.extend_from_slice(user.as_bytes());
        request.push(pass.len() as u8);
        request.extend_from_slice(pass.as_bytes());
        server.write_all(&request).await?;

        let mut response = [0; 2];
        server.read_exact(&mut response).await?;

        match response[1] {
            0 => Client::request(server).await?,
            _ => return Err(anyhow!("Authentication failed.")),
        }

        Ok(())
    }

    async fn request(server: &mut TcpStream) -> Result<()> {
        let ip = Ipv4Addr::new(8, 8, 8, 8);
        let port: u16 = 80;

        let mut request = vec![5, 1, 0, 1];
        request.extend_from_slice(&ip.octets());
        request.extend_from_slice(&port.to_be_bytes());
        server.write_all(&request).await?;

        let mut response = vec![0; 2];
        server.read_exact(&mut response).await?;

        match response[1] {
            0 => Ok(()),
            _ => Err(anyhow!("Request failed.")),
        }
    }
}
