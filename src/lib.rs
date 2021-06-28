use async_std::{
    net::{TcpListener, TcpStream},
    task,
};
use eyre::Result;
use futures::{channel::mpsc::unbounded, future, pin_mut, StreamExt, TryStreamExt};
use main_state::WrappedMainState;
use std::net::SocketAddr;

use crate::{
    main_state::MainState,
    message::{IncommingMessage, OutgoingMessage},
};

mod card;
mod command;
mod deck;
mod main_state;
mod message;
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
            let incomming_message: IncommingMessage =
                serde_json::from_str(message.to_text().unwrap()).unwrap();
            match incomming_message.command {
                command::Command::CreateGame => {
                    let mut state = main_state.lock().unwrap();
                    let code = state.create_room(address).unwrap();
                    let mut message = OutgoingMessage::default();
                    if let Some(draw_deck_size) = state.get_draw_deck_size(&code) {
                        message.set_draw_deck_size(draw_deck_size);
                    }
                    message.set_room_code(code);
                    message
                        .set_message("Game created, invite people with the room code above".into());
                    message.set_command(incomming_message.command);
                    state.send_message_to_address(&address, &message).unwrap();
                }
                command::Command::JoinRoom => {
                    let mut state = main_state.lock().unwrap();
                    let mut message = OutgoingMessage::default();
                    if let Some(code) = &incomming_message.room_code {
                        // if let Err(error) = state.join_room(code, address) {
                        //     message.set_error(error.to_string());
                        // } else {
                        //     message.set_room_code(code.clone());
                        //     message.set_message("Room joined!".into());
                        // }
                        match state.join_room(code, address) {
                            Ok(draw_deck_size) => {
                                message.set_room_code(code.clone());
                                message.set_message("Room joined!".into());
                                message.set_draw_deck_size(draw_deck_size);
                            }
                            Err(error) => message.set_error(error.to_string()),
                        }
                    } else {
                        message.set_error("Please set a room code".into());
                    }
                    state.send_message_to_address(&address, &message).unwrap();
                }
                command::Command::Chat => {
                    let mut state = main_state.lock().unwrap();
                    let mut outgoing_message = OutgoingMessage::default();
                    let room_code = &incomming_message.room_code.unwrap();
                    outgoing_message.set_room_code(room_code.clone());
                    outgoing_message.set_chat_message(incomming_message.message.unwrap());
                    if let Some(draw_deck_size) = state.get_draw_deck_size(room_code) {
                        outgoing_message.set_draw_deck_size(draw_deck_size);
                    }
                    state
                        .broadcast_to_room(room_code, &outgoing_message)
                        .unwrap();
                }
                command::Command::DrawCard => {
                    let mut state = main_state.lock().unwrap();
                    let mut outgoing_message = OutgoingMessage::default();
                    let room_code = &incomming_message.room_code.unwrap();
                    outgoing_message.set_room_code(room_code.clone());
                    outgoing_message.set_command(incomming_message.command);
                    if let Some(drawn_card) = state.handle_draw_card(room_code, address) {
                        outgoing_message.set_card(drawn_card);
                    }
                    if let Some(deck_size) = state.get_draw_deck_size(room_code) {
                        outgoing_message.set_draw_deck_size(deck_size);
                    }
                    state
                        .send_message_to_address(&address, &outgoing_message)
                        .unwrap();
                    let mut broadcast_message = outgoing_message.clone();
                    broadcast_message.remove_card();
                    state
                        .broadcast_to_everyone_else(room_code, &address, &broadcast_message)
                        .unwrap();
                }
                command::Command::None => {}
            }
            // let peers = peer_map.lock().unwrap();
            // let broadcast_recipients = peers
            //     .iter()
            //     .filter(|(peer_address, _)| peer_address != &&address)
            //     .map(|(_, sender)| sender);

            // for sender in broadcast_recipients {
            //     sender.unbounded_send(message.clone()).unwrap();
            // }

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
