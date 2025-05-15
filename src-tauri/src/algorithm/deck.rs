use crate::algorithm::card_data::CardData;
use crate::algorithm::card_suits::CardSuit;
use crate::algorithm::faces::CardFace;

pub const DECK: [CardData; 52] = get_deck_ace_high();
pub const fn get_deck_ace_high() -> [CardData; 52] {
    let mut deck = [CardData {
        card_suit: CardSuit::Clubs,
        card_face: CardFace::Ace,
    }; 52];
    let mut i = 0;
    let mut suit_index = 0;
    while suit_index < 4 {
        let suit = CardSuit::from_index(suit_index);
        let mut value = 2;
        while value <= 14 {
            deck[i] = CardData {
                card_suit: suit,
                card_face: CardFace::from_value(value),
            };
            i += 1;
            value += 1;
        }
        suit_index += 1;
    }
    deck
}