use bincode::Config;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::app::App;
use crate::backend;

async fn peer(args: Vec<String>, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {

    
    if args.len() == 1 {
        println!("Initilized as server");
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        while let Ok((stream, _)) = listener.accept().await {
            println!("Connected!");
            backend::server::handle_stream(stream).await?;
        }
    } else {
        println!("Trying to init as client");
        let addr = args[1].clone();

        if addr.contains("://") {
            let host: url::Url = addr.parse().unwrap();
            let socket = host.socket_addrs(|| Some(80))?;
            let mut stream = TcpStream::connect(socket[0]).await?;
            let _ = stream.write_all(&bincode::serialize(&backend::message::Message::JoinNetwork(
                "hello".to_owned(),
            ))?);
        } else {
            let socket: SocketAddr = args[1].clone().parse()?;
            let stream = TcpStream::connect(socket).await?;
            backend::client::handle_stream(stream).await?;
        }
    }

    Ok(())
}