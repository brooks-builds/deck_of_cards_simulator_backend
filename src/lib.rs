use async_std::{
    net::{TcpListener, TcpStream},
    task,
};
use eyre::Result;
use futures::{channel::mpsc::unbounded, future, pin_mut, StreamExt, TryStreamExt};
use main_state::WrappedMainState;
use std::net::SocketAddr;

use crate::{main_state::MainState, message::IncomingMessage};

mod card;
mod command;
mod deck;
mod main_state;
mod message;
mod player;
mod room;

async fn handle_connection(
    main_state: WrappedMainState,
    raw_stream: TcpStream,
    address: SocketAddr,
) {
    println!("incoming connection from {}", address);
    let websocket = async_tungstenite::accept_async(raw_stream).await.unwrap();
    println!("websocket connection established: {}", address);
    let (sender, receiver) = unbounded();
    main_state.lock().unwrap().add_client(address, sender);
    let (websocket_out, websocket_in) = websocket.split();
    let broadcast_incoming = websocket_in
        .try_filter(|message| future::ready(!message.is_close()))
        .try_for_each(|message| {
            // received message and now we can handle it
            let incoming_message: IncomingMessage =
                serde_json::from_str(message.to_text().unwrap()).unwrap();
            let mut state = main_state.lock().unwrap();
            match incoming_message.command {
                command::Command::CreateGame => {
                    state.create_game(address, incoming_message).unwrap()
                }
                command::Command::JoinRoom => state.join_room(incoming_message, address).unwrap(),
                command::Command::Chat => state.handle_chat(incoming_message, address).unwrap(),
                command::Command::DrawCard => {
                    state.handle_draw_card(incoming_message, address).unwrap()
                }
            }

            future::ok(())
        });
    let receive_from_others = receiver.map(Ok).forward(websocket_out);
    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;
}

pub async fn run() -> Result<()> {
    let address = "127.0.0.1:9001".to_string();
    let main_state = MainState::new_wrapped();
    let listener = TcpListener::bind(&address).await?;
    println!("websocket server listening on {}", address);

    while let Ok((stream, address)) = listener.accept().await {
        task::spawn(handle_connection(main_state.clone(), stream, address));
    }
    Ok(())
}
