use std::net::SocketAddr;

use crate::deck::Deck;

#[derive(Debug)]
pub struct Room {
    addresses: Vec<SocketAddr>,
    deck: Deck,
}

impl Room {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            addresses: vec![address],
            deck: Deck::new(),
        }
    }

    pub fn join(&mut self, address: SocketAddr) {
        self.addresses.push(address);
    }

    pub fn get_addresses(&self) -> &[SocketAddr] {
        &self.addresses
    }

    pub fn get_draw_deck_size(&self) -> u8 {
        self.deck.get_draw_deck_size()
    }

    pub fn draw(&mut self, address: SocketAddr) {
        self.deck.draw(address);
    }
}
