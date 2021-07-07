use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub struct Card {
    pub suite: Suite,
    pub value: Value,
    visible: bool,
}

impl Card {
    pub fn new(suite: Suite, value: Value) -> Self {
        Self {
            suite,
            value,
            visible: false,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn card_data(&self) -> CardData {
        if self.visible {
            CardData {
                suite: Some(self.suite),
                value: Some(self.value),
                visible: true,
            }
        } else {
            CardData {
                suite: None,
                value: None,
                visible: false,
            }
        }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    pub suite: Option<Suite>,
    pub value: Option<Value>,
    pub visible: bool,
}
