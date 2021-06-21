use async_std::{
    net::{TcpListener, TcpStream},
    task,
};
use async_tungstenite::tungstenite::Message;
use eyre::Result;
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future, pin_mut, StreamExt, TryStreamExt,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

type Sender = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Sender>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, address: SocketAddr) {
    println!("incoming connection from {}", address);
    let websocket = async_tungstenite::accept_async(raw_stream).await.unwrap();
    println!("websocket connection established: {}", address);
    let (sender, receiver) = unbounded();
    peer_map.lock().unwrap().insert(address, sender);
    let (websocket_out, websocket_in) = websocket.split();
    let broadcast_incoming = websocket_in
        .try_filter(|message| {
            dbg!(message);
            future::ready(!message.is_close())
        })
        .try_for_each(|message| {
            println!(
                "Received message from {}: {}",
                address,
                message.to_text().unwrap()
            );
            let peers = peer_map.lock().unwrap();
            let broadcast_recipients = peers
                .iter()
                .filter(|(peer_address, _)| peer_address != &&address)
                .map(|(_, sender)| sender);

            for sender in broadcast_recipients {
                sender.unbounded_send(message.clone()).unwrap();
            }

            future::ok(())
        });
    let receive_from_others = receiver.map(Ok).forward(websocket_out);
    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;
}

async fn run() -> Result<()> {
    let address = "127.0.0.1:9001".to_string();
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind(&address).await?;
    println!("websocket server listening on {}", address);

    while let Ok((stream, address)) = listener.accept().await {
        task::spawn(handle_connection(state.clone(), stream, address));
    }
    Ok(())
}
fn main() -> Result<()> {
    task::block_on(run())
}
