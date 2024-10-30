use std::sync::{Arc, Mutex};

use crate::backend::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_stream(
    mut stream: TcpStream,
    app: Arc<Mutex<crate::App>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: [u8; 1024] = [0; 1024];
    stream
        .write_all(&bincode::serialize(&message::Message::Incriment)?)
        .await?;
    loop {
        match stream.read(&mut buf).await {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    let message: message::Message = bincode::deserialize(&buf)?;
                }
            }
            Err(err) => {}
        }
        stream
            .write_all(&bincode::serialize(&message::Message::Incriment)?)
            .await?;
    }
    Ok(())
}
