use bincode::Config;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    JoinNetwork(String),
    LeaveNetwork,
    SendMessage(String),
}

impl Message {
    pub fn parse(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data).map_err(|e| e.into())
    }

    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self).map_err(|e| e.into())
    }
}

async fn send_message(
    address: SocketAddr,
    message: &Message,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(address).await?;

    // Serialize the message
    let serialized_message = message.serialize()?;
    println!("{:x?}", serialized_message);

    // Write the serialized message
    stream.write_all(&serialized_message).await?;

    println!("Sent message: {:?}", message);
    let mut buf = [0;1024];
    let _ = stream.read(&mut buf).await;
    let res = Message::parse(&buf);
    println!("Recieved: {:?}", res);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    let message = Message::JoinNetwork("Hello, Server!".to_string());

    send_message(address, &message).await?;
    
    Ok(())
}
