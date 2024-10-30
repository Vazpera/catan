use crate::backend::*;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub async fn handle_stream(
    mut stream: TcpStream,
    app: Arc<Mutex<crate::App>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        match stream.read(&mut buf).await {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    let message: message::Message = bincode::deserialize(&buf)?;
                    use message::Message::*;
                    println!("quitting");
                    let mut held = app.lock().unwrap();
                    match message {
                        Incriment => {
                            held.running = false;
                        }
                        _ => {}
                    }
                    drop(held);
                }
            }
            Err(err) => {}
        }
    }
    Ok(())
}
