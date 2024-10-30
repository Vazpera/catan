use crate::backend::*;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub async fn handle_stream(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: [u8; 1024] = [0; 1024];
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
    }
    Ok(())
}
