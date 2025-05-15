use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardFace {
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
    Ace,
}



impl Default for CardFace {
    fn default() -> Self {
        CardFace::Ace
    }
}

impl CardFace {
    pub const fn from_value(value: u8) -> Self {
        match value {
            1 | 11 => CardFace::Ace,
            2 => CardFace::Two,
            3 => CardFace::Three,
            4 => CardFace::Four,
            5 => CardFace::Five,
            6 => CardFace::Six,
            7 => CardFace::Seven,
            8 => CardFace::Eight,
            9 => CardFace::Nine,
            10 => CardFace::Ten,
            12 => CardFace::Jack,
            13 => CardFace::Queen,
            14 => CardFace::King,
            _ => panic!("invalid value"),
        }
    }
    pub fn value(&self) -> u8 {
        match self {
            CardFace::Two => 2,
            CardFace::Three => 3,
            CardFace::Four => 4,
            CardFace::Five => 5,
            CardFace::Six => 6,
            CardFace::Seven => 7,
            CardFace::Eight => 8,
            CardFace::Nine => 9,
            CardFace::Ten => 10,
            CardFace::Ace => 11,
            CardFace::Jack => 11,
            CardFace::Queen => 12,
            CardFace::King => 13,
        }
    }
}

impl<T> From<T> for CardFace
where
    T: Into<u8>,
{
    fn from(value: T) -> Self {
        Self::from_value(value.into())
    }
}
