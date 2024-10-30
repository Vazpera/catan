use crate::app::App;
use crate::backend;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

pub async fn peer(
    args: Vec<String>,
    app: Arc<Mutex<App>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let distributed_app = app.clone();
    let mut held = app.lock().unwrap();
    if args.len() == 1 {
        held.is_host = true;
        drop(held);
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        while let Ok((stream, _)) = listener.accept().await {
            backend::server::handle_stream(stream, distributed_app.clone()).await?;
        }
    } else {
        drop(held);
        let addr = args[1].clone();

        if addr.contains("://") {
            let host: url::Url = addr.parse().unwrap();
            let socket = host.socket_addrs(|| Some(80))?;
            let stream = TcpStream::connect(socket[0]).await?;
            backend::client::handle_stream(stream, app.clone()).await?;
        } else {
            let socket: SocketAddr = args[1].clone().parse()?;
            let stream = TcpStream::connect(socket).await?;
            backend::client::handle_stream(stream, app.clone()).await?;
        }
    }

    Ok(())
}
