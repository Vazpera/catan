use bincode::Config;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::backend::*;

pub async fn handle_stream(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: [u8; 1024] = [0; 1024];
    stream.write_all(&bincode::serialize(&message::Message::JoinNetwork("hello".to_owned()))?).await?;
    loop {
        match stream.read(&mut buf).await {
            Ok(n) => {
                println!("Recieved message");
                if n == 0 {
                    println!("Connection closed");
                    break;
                } else {
                    let message: message::Message = bincode::deserialize(&buf)?;
                    println!("Recieved: {:?}", message);
                }
            }
            Err(err) => {
                eprintln!("Error when receiving message: {err}");
            }
        }
        stream.write_all(&bincode::serialize(&message::Message::Incriment)?);
    }
    Ok(())
}
