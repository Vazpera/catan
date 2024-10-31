use crate::app::App;
use crate::backend;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

use crate::backend::*;
use futures::{FutureExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn handle_stream(
    mut stream: TcpStream,
    app: Arc<crate::App>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: [u8; 1024] = [0; 1024];
    let mut tick = 0;
    loop {
        let mut recv = app.queued_instructions.1.lock().unwrap();
        let out_stream = recv.next().boxed();
        let in_stream = stream.read(&mut buf);
        let _ = tokio::select! {
            res = in_stream => {
                match res {
                    Ok(n) => {
                        if n == 0 || !*app.running.lock().unwrap() {
                            *app.running.lock().unwrap() = false;
                            break;
                        } else {
                            let message: message::Message = bincode::deserialize(&buf)?;
                            message.eval(app.clone());
                        }
                    }
                    Err(err) => {}
                }
                stream
                    .write_all(&bincode::serialize(&message::Message::Incriment)?)
                    .await?;

                    tick += 1;
                    if tick % 10 == 0 {
                        stream.write_all(&bincode::serialize(&message::Message::Ping)?).await?;
                    }
            }
            value = out_stream => {
                match value {
                    Some(v) => {
                        stream.write_all(&bincode::serialize(&v)?).await?;
                    }
                    None => {}
                }
            }

        };
    }
    Ok(())
}

pub async fn peer(args: Vec<String>, app: Arc<App>) -> Result<(), Box<dyn std::error::Error>> {
    let distributed_app = app.clone();
    if args.len() == 1 {
        *app.is_host.lock().unwrap() = true;
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        while let Ok((stream, _)) = listener.accept().await {
            handle_stream(stream, distributed_app.clone()).await?;
        }
    } else {
        let addr = args[1].clone();

        if addr.contains("://") {
            let host: url::Url = addr.parse().unwrap();
            let socket = host.socket_addrs(|| Some(80))?;
            let stream = TcpStream::connect(socket[0]).await?;
           handle_stream(stream, app.clone()).await?;
        } else {
            let socket: SocketAddr = args[1].clone().parse()?;
            let stream = TcpStream::connect(socket).await?;
           handle_stream(stream, app.clone()).await?;
        }
    }

    Ok(())
}
