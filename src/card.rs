use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Card {
    pub suite: Suite,
    pub value: Value,
}

impl Card {
    pub fn new(suite: Suite, value: Value) -> Self {
        Self { suite, value }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
pub enum Suite {
    Club,
    Heart,
    Diamond,
    Spade,
}

impl Suite {
    pub fn all() -> [Suite; 4] {
        [Self::Club, Self::Heart, Self::Diamond, Self::Spade]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
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

impl Value {
    pub fn all() -> [Value; 13] {
        [
            Self::Ace,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::Ten,
            Self::Jack,
            Self::Queen,
            Self::King,
        ]
    }
}
