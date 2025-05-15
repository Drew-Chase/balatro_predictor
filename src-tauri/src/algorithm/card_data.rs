use crate::algorithm::card_suits::CardSuit;
use crate::algorithm::faces::CardFace;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
#[derive(Serialize, Deserialize, Copy, Clone, Default)]
pub struct CardData {
    pub card_suit: CardSuit,
    pub card_face: CardFace,
}

impl CardData {
    pub fn is_ace(&self) -> bool {
        self.card_face == CardFace::Ace
    }
    pub fn is_face_card(&self) -> bool {
        self.card_face == CardFace::King
            || self.card_face == CardFace::Queen
            || self.card_face == CardFace::Jack
    }
}

pub trait BalatroPredictor {
    fn calculate_high_card_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32>;
    fn calculate_pair_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32>;
    fn calculate_two_pair_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32>;
    fn calculate_three_of_a_kind_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32>;
    fn calculate_straight_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32>;
    fn calculate_flush_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32>;
    fn calculate_full_house_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32>;
    fn calculate_four_of_a_kind_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32>;
    fn calculate_straight_flush_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32>;
    fn calculate_royal_flush_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32>;
}

impl Debug for CardData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CardData {{ card_suit: {:?}, card_face: {:?} ({}) }}",
            self.card_suit,
            self.card_face,
            self.card_face.value()
        )
    }
}
