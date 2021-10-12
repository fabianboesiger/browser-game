mod message;
mod transport;
mod event;

use tokio::net::TcpListener;
use transport::Transport;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let transport = Transport::new().await;

    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(&addr).await.expect("Can't bind TCP listener.");
    log::info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        //let peer = stream.peer_addr().expect("connected streams should have a peer address");
        //log::info!("Peer address: {}", peer);
        transport.connect(stream).await;
        //tokio::spawn(accept_connection(stream, transport.clone()));
    }
}