use crate::app::App;
use crate::backend;
use crate::backend::*;
use futures::{FutureExt, StreamExt};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

pub async fn handle_stream(
    mut stream: TcpStream,
    app: Arc<crate::App>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        let mut recv = app.message_stream.1.lock().unwrap();
        let out_stream = recv.next().boxed();
        let in_stream = stream.read(&mut buf);
        if *app.running.lock().unwrap() == false {
            break;
        }
        let _ = tokio::select! {
            res = in_stream => {
                match res {
                    Ok(n) => {
                        if n == 0 || !*app.running.lock().unwrap() {
                            if *app.is_host.lock().unwrap() {
                                *app.connections.lock().unwrap() -= 1;
                            }
                            break;
                        } else {
                            let message: message::Message = bincode::deserialize(&buf)?;
                            message.eval(app.clone()).await;
                        }
                    }
                    Err(err) => {}
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
            *app.connections.lock().unwrap() += 1;

            handle_stream(stream, distributed_app.clone()).await?;
        }
    } else {
        let socket: SocketAddr = args[1].clone().parse()?;
        let stream = TcpStream::connect(socket).await?;
        handle_stream(stream, app.clone())
            .await
            .expect("Couldn't connect!");
    }

    Ok(())
}
