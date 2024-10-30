use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

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

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = vec![0; 1024];

    loop {
        match stream.read(&mut buffer).await {
            Ok(n) => {
                if n == 0 {
                    println!("Connection closed");
                    return;
                }
                println!("{:?}", stream.local_addr());

                let message = Message::parse(&buffer[..n]).unwrap();
                println!("Received message: {:?}", message);

                // Process the message here

                // Send response
                let response =
                    Message::SendMessage(format!("Hello, Client at {:?}!", stream.local_addr()));
                let serialized_response = response.serialize().unwrap();
                stream.write_all(&serialized_response).await.unwrap();
            }
            Err(_) => break,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
    Ok(())
}
