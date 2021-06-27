use std::net::SocketAddr;

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub suite: Suite,
    pub value: Value,
    pub owner: Owner,
}

impl Card {
    pub fn new(suite: Suite, value: Value, owner: Owner) -> Self {
        Self {
            suite,
            value,
            owner,
        }
    }

    pub fn is_owned_by(&self, owner: Owner) -> bool {
        self.owner == owner
    }

    pub fn change_owner(&mut self, new_owner: Owner) {
        self.owner = new_owner;
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Suite {
    Club,
    Heart,
    Diamond,
    Spade,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Owner {
    Draw,
    Discard,
    Player(SocketAddr),
}
