use std::net::SocketAddr;

use crate::card::{Card, Owner};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = vec![];
        let card = Card::new(
            crate::card::Suite::Club,
            crate::card::Value::Ace,
            Owner::Draw,
        );
        let ace_of_hearts = Card::new(
            crate::card::Suite::Heart,
            crate::card::Value::Ace,
            Owner::Draw,
        );
        cards.push(card);
        cards.push(ace_of_hearts);

        Self { cards }
    }

    pub fn get_draw_deck_size(&self) -> u8 {
        self.cards
            .iter()
            .filter(|card| card.is_owned_by(Owner::Draw))
            .count() as u8
    }

    pub fn draw(&mut self, address: SocketAddr) -> Option<Card> {
        let owner = Owner::Player(address);
        if let Some(card) = self
            .cards
            .iter_mut()
            .find(|card| card.is_owned_by(Owner::Draw))
        {
            card.change_owner(owner);
            Some(card.clone())
        } else {
            None
        }
    }
}
