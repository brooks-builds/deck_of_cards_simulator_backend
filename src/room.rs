use std::net::SocketAddr;

#[derive(Debug)]
pub struct Room {
    addresses: Vec<SocketAddr>,
    draw_deck_size: u8,
}

impl Room {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            addresses: vec![address],
            draw_deck_size: 0,
        }
    }

    pub fn join(&mut self, address: SocketAddr) {
        self.addresses.push(address);
    }

    pub fn get_addresses(&self) -> &[SocketAddr] {
        &self.addresses
    }

    pub fn get_draw_deck_size(&self) -> u8 {
        self.draw_deck_size
    }
}
