use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl CardSuit {
    pub const fn from_index(index: u8) -> Self {
        match index {
            0 => CardSuit::Hearts,
            1 => CardSuit::Diamonds,
            2 => CardSuit::Clubs,
            3 => CardSuit::Spades,
            _ => panic!("invalid suit"),
        }
    }
}


impl Default for CardSuit {
    fn default() -> Self {
        Self::Clubs
    }   
}

impl<T> From<T> for CardSuit where T: Into<u8> {
    fn from(value: T) -> Self {
        match value.into() {
            0 => Self::Clubs,
            1 => Self::Diamonds,
            2 => Self::Hearts,
            3 => Self::Spades,
            _ => panic!("Invalid card suit value"),
        }
    }   
}
