use crate::algorithm::card_data::{BalatroPredictor, CardData};
use crate::algorithm::deck::DECK;
use crate::algorithm::faces::CardFace;
use crate::algorithm::card_suits::CardSuit;
use std::collections::{HashMap, HashSet};

// Helper functions for hand detection
fn count_faces(cards: &[CardData]) -> HashMap<CardFace, usize> {
    let mut face_counts = HashMap::new();
    for card in cards {
        *face_counts.entry(card.card_face).or_insert(0) += 1;
    }
    face_counts
}

fn count_suits(cards: &[CardData]) -> HashMap<CardSuit, usize> {
    let mut suit_counts = HashMap::new();
    for card in cards {
        *suit_counts.entry(card.card_suit).or_insert(0) += 1;
    }
    suit_counts
}

fn has_pair(cards: &[CardData]) -> bool {
    let face_counts = count_faces(cards);
    face_counts.values().any(|&count| count >= 2)
}

fn has_two_pair(cards: &[CardData]) -> bool {
    let face_counts = count_faces(cards);
    let pairs = face_counts.values().filter(|&&count| count >= 2).count();
    pairs >= 2
}

fn has_three_of_a_kind(cards: &[CardData]) -> bool {
    let face_counts = count_faces(cards);
    face_counts.values().any(|&count| count >= 3)
}

fn has_straight(cards: &[CardData]) -> bool {
    if cards.len() < 5 {
        return false;
    }

    // Get unique face values
    let mut values: Vec<u8> = cards.iter()
        .map(|card| card.card_face.value())
        .collect();
    values.sort();
    values.dedup();

    // Check for 5 consecutive values
    for window in values.windows(5) {
        if window[4] - window[0] == 4 {
            return true;
        }
    }

    // Special case for A-2-3-4-5 straight (Ace can be low)
    if values.contains(&2) && values.contains(&3) && values.contains(&4) && 
       values.contains(&5) && values.contains(&11) { // 11 is Ace
        return true;
    }

    false
}

fn has_flush(cards: &[CardData]) -> bool {
    let suit_counts = count_suits(cards);
    suit_counts.values().any(|&count| count >= 5)
}

fn has_full_house(cards: &[CardData]) -> bool {
    let face_counts = count_faces(cards);
    let has_three = face_counts.values().any(|&count| count >= 3);
    let pairs = face_counts.values().filter(|&&count| count >= 2).count();

    has_three && pairs >= 2
}

fn has_four_of_a_kind(cards: &[CardData]) -> bool {
    let face_counts = count_faces(cards);
    face_counts.values().any(|&count| count >= 4)
}

fn has_straight_flush(cards: &[CardData]) -> bool {
    if cards.len() < 5 {
        return false;
    }

    // Group cards by suit
    let mut cards_by_suit: HashMap<CardSuit, Vec<CardData>> = HashMap::new();
    for card in cards {
        cards_by_suit.entry(card.card_suit).or_insert_with(Vec::new).push(*card);
    }

    // Check each suit group for a straight
    for (_, suit_cards) in cards_by_suit {
        if suit_cards.len() >= 5 && has_straight(&suit_cards) {
            return true;
        }
    }

    false
}

fn has_royal_flush(cards: &[CardData]) -> bool {
    if cards.len() < 5 {
        return false;
    }

    // Group cards by suit
    let mut cards_by_suit: HashMap<CardSuit, Vec<CardData>> = HashMap::new();
    for card in cards {
        cards_by_suit.entry(card.card_suit).or_insert_with(Vec::new).push(*card);
    }

    // Check each suit group for a royal flush (10, J, Q, K, A of same suit)
    for (_, suit_cards) in cards_by_suit {
        if suit_cards.len() >= 5 {
            let has_ten = suit_cards.iter().any(|card| card.card_face == CardFace::Ten);
            let has_jack = suit_cards.iter().any(|card| card.card_face == CardFace::Jack);
            let has_queen = suit_cards.iter().any(|card| card.card_face == CardFace::Queen);
            let has_king = suit_cards.iter().any(|card| card.card_face == CardFace::King);
            let has_ace = suit_cards.iter().any(|card| card.card_face == CardFace::Ace);

            if has_ten && has_jack && has_queen && has_king && has_ace {
                return true;
            }
        }
    }

    false
}

// Helper function to calculate probability of getting a pair or better
fn calculate_pair_or_better_probability(current_cards: &[CardData], available_cards: &[CardData]) -> f32 {
    // This is a simplified calculation
    // In reality, we'd need to calculate the probability of drawing cards that complete each hand type

    let face_counts = count_faces(current_cards);

    // Calculate probability of drawing to complete a pair
    let mut pair_probability = 0.0;
    for (&face, &count) in &face_counts {
        if count == 1 {
            // Count how many cards of this face are available
            let matching_cards = available_cards.iter()
                .filter(|card| card.card_face == face)
                .count() as f32;

            // Probability of drawing at least one of these cards
            if !available_cards.is_empty() {
                pair_probability += matching_cards / available_cards.len() as f32;
            }
        }
    }

    // Simplification: cap at 1.0
    pair_probability.min(1.0)
}

impl BalatroPredictor for Vec<CardData> {
    fn calculate_high_card_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate high card"));
        }

        // Get the cards that are already in hand or discarded
        let mut unavailable_cards = self.clone();
        if let Some(discarded) = discarded_cards {
            unavailable_cards.extend(discarded);
        }

        // Check if we already have a better hand than high card
        if has_pair(&unavailable_cards) || has_three_of_a_kind(&unavailable_cards) || 
           has_straight(&unavailable_cards) || has_flush(&unavailable_cards) || 
           has_full_house(&unavailable_cards) || has_four_of_a_kind(&unavailable_cards) || 
           has_straight_flush(&unavailable_cards) || has_royal_flush(&unavailable_cards) {
            return Ok(0.0);
        }

        // Calculate probability of drawing only high cards
        // For high card, we need to avoid getting any pairs, straights, or flushes
        // This is a simplification - we're calculating the probability of not getting any better hand
        let available_cards: Vec<CardData> = DECK.iter()
            .filter(|card| !unavailable_cards.contains(card))
            .cloned()
            .collect();

        // If we have no cards left to draw, return 0
        if available_cards.is_empty() {
            return Ok(0.0);
        }

        // Calculate probability of not getting any better hand
        // This is a simplification - in reality, we'd need to calculate the probability
        // of not getting any pairs, straights, flushes, etc.
        let total_remaining_cards = available_cards.len() as f32;

        // For simplicity, we'll estimate the probability based on the current hand composition
        // The more diverse the hand (different faces, different suits), the higher the chance of high card
        let face_counts = count_faces(&unavailable_cards);
        let suit_counts = count_suits(&unavailable_cards);

        // If we have cards of the same face, we already have a pair or better
        let has_duplicates = face_counts.values().any(|&count| count > 1);
        if has_duplicates {
            return Ok(0.0);
        }

        // Calculate probability based on remaining cards
        // This is a rough approximation
        let high_card_probability = 1.0 - (calculate_pair_or_better_probability(&unavailable_cards, &available_cards));

        Ok(high_card_probability.max(0.0).min(1.0))
    }

    fn calculate_pair_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate pair"));
        }

        // Get the cards that are already in hand or discarded
        let mut unavailable_cards = self.clone();
        if let Some(discarded) = discarded_cards {
            unavailable_cards.extend(discarded);
        }

        // Check if we already have a pair
        if has_pair(&unavailable_cards) {
            return Ok(1.0);
        }

        // Check if we already have a better hand than pair
        if has_two_pair(&unavailable_cards) || has_three_of_a_kind(&unavailable_cards) || 
           has_straight(&unavailable_cards) || has_flush(&unavailable_cards) || 
           has_full_house(&unavailable_cards) || has_four_of_a_kind(&unavailable_cards) || 
           has_straight_flush(&unavailable_cards) || has_royal_flush(&unavailable_cards) {
            return Ok(0.0);
        }

        // Calculate probability of getting a pair
        let available_cards: Vec<CardData> = DECK.iter()
            .filter(|card| !unavailable_cards.contains(card))
            .cloned()
            .collect();

        // If we have no cards left to draw, return 0
        if available_cards.is_empty() {
            return Ok(0.0);
        }

        // Calculate probability of drawing to complete a pair
        let face_counts = count_faces(&unavailable_cards);

        let mut pair_probability = 0.0;
        for (&face, &count) in &face_counts {
            if count == 1 {
                // Count how many cards of this face are available
                let matching_cards = available_cards.iter()
                    .filter(|card| card.card_face == face)
                    .count() as f32;

                // Probability of drawing at least one of these cards
                if !available_cards.is_empty() {
                    pair_probability += matching_cards / available_cards.len() as f32;
                }
            }
        }

        Ok(pair_probability.max(0.0).min(1.0))
    }

    fn calculate_two_pair_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate two pair"));
        }

        // Get the cards that are already in hand or discarded
        let mut unavailable_cards = self.clone();
        if let Some(discarded) = discarded_cards {
            unavailable_cards.extend(discarded);
        }

        // Check if we already have two pair
        if has_two_pair(&unavailable_cards) {
            return Ok(1.0);
        }

        // Check if we already have a better hand than two pair
        if has_three_of_a_kind(&unavailable_cards) || has_straight(&unavailable_cards) || 
           has_flush(&unavailable_cards) || has_full_house(&unavailable_cards) || 
           has_four_of_a_kind(&unavailable_cards) || has_straight_flush(&unavailable_cards) || 
           has_royal_flush(&unavailable_cards) {
            return Ok(0.0);
        }

        // Calculate probability of getting two pair
        let available_cards: Vec<CardData> = DECK.iter()
            .filter(|card| !unavailable_cards.contains(card))
            .cloned()
            .collect();

        // If we have no cards left to draw, return 0
        if available_cards.is_empty() {
            return Ok(0.0);
        }

        // Calculate probability based on current hand
        let face_counts = count_faces(&unavailable_cards);

        // If we already have one pair, calculate probability of getting another pair
        let pairs = face_counts.values().filter(|&&count| count >= 2).count();
        if pairs == 1 {
            // We have one pair, need to get another pair
            let mut second_pair_probability = 0.0;
            for (&face, &count) in &face_counts {
                if count == 1 {
                    // Count how many cards of this face are available
                    let matching_cards = available_cards.iter()
                        .filter(|card| card.card_face == face)
                        .count() as f32;

                    // Probability of drawing at least one of these cards
                    if !available_cards.is_empty() {
                        second_pair_probability += matching_cards / available_cards.len() as f32;
                    }
                }
            }

            return Ok(second_pair_probability.max(0.0).min(1.0));
        } else {
            // We have no pairs yet, need to get two pairs
            // This is a more complex calculation and would require considering combinations
            // For simplicity, we'll use a rough approximation
            let mut two_pair_probability = 0.0;

            // Probability of getting at least two pairs from the available cards
            // This is a simplified calculation
            if available_cards.len() >= 4 {
                // Count how many different faces we have in our hand
                let different_faces = face_counts.len();

                // The more different faces we have, the higher the chance of getting two pairs
                two_pair_probability = (different_faces as f32 / 13.0) * 0.5;
            }

            return Ok(two_pair_probability.max(0.0).min(1.0));
        }
    }

    fn calculate_three_of_a_kind_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!(
                "Not enough cards to calculate three of a kind"
            ));
        }

        // Get the cards that are already in hand or discarded
        let mut unavailable_cards = self.clone();
        if let Some(discarded) = discarded_cards {
            unavailable_cards.extend(discarded);
        }

        // Check if we already have three of a kind
        if has_three_of_a_kind(&unavailable_cards) {
            return Ok(1.0);
        }

        // Check if we already have a better hand than three of a kind
        if has_straight(&unavailable_cards) || has_flush(&unavailable_cards) || 
           has_full_house(&unavailable_cards) || has_four_of_a_kind(&unavailable_cards) || 
           has_straight_flush(&unavailable_cards) || has_royal_flush(&unavailable_cards) {
            return Ok(0.0);
        }

        // Calculate probability of getting three of a kind
        let available_cards: Vec<CardData> = DECK.iter()
            .filter(|card| !unavailable_cards.contains(card))
            .cloned()
            .collect();

        // If we have no cards left to draw, return 0
        if available_cards.is_empty() {
            return Ok(0.0);
        }

        // Calculate probability based on current hand
        let face_counts = count_faces(&unavailable_cards);

        // If we already have a pair, calculate probability of getting another card of the same face
        let mut three_of_a_kind_probability = 0.0;
        for (&face, &count) in &face_counts {
            if count == 2 {
                // We have a pair, need one more card of this face
                let matching_cards = available_cards.iter()
                    .filter(|card| card.card_face == face)
                    .count() as f32;

                // Probability of drawing at least one of these cards
                if !available_cards.is_empty() {
                    three_of_a_kind_probability += matching_cards / available_cards.len() as f32;
                }
            } else if count == 1 {
                // We have one card, need two more of this face
                let matching_cards = available_cards.iter()
                    .filter(|card| card.card_face == face)
                    .count() as f32;

                // Probability of drawing at least two of these cards
                // This is a simplified calculation
                if available_cards.len() >= 2 && matching_cards >= 2.0 {
                    three_of_a_kind_probability += (matching_cards / available_cards.len() as f32) * 0.3;
                }
            }
        }

        Ok(three_of_a_kind_probability.max(0.0).min(1.0))
    }

    fn calculate_straight_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate straight"));
        }

        // Get the cards that are already in hand or discarded
        let mut unavailable_cards = self.clone();
        if let Some(discarded) = discarded_cards {
            unavailable_cards.extend(discarded);
        }

        // Check if we already have a straight
        if has_straight(&unavailable_cards) {
            return Ok(1.0);
        }

        // Check if we already have a better hand than straight
        if has_flush(&unavailable_cards) || has_full_house(&unavailable_cards) || 
           has_four_of_a_kind(&unavailable_cards) || has_straight_flush(&unavailable_cards) || 
           has_royal_flush(&unavailable_cards) {
            return Ok(0.0);
        }

        // Calculate probability of getting a straight
        let available_cards: Vec<CardData> = DECK.iter()
            .filter(|card| !unavailable_cards.contains(card))
            .cloned()
            .collect();

        // If we have no cards left to draw, return 0
        if available_cards.is_empty() {
            return Ok(0.0);
        }

        // Get unique face values
        let mut values: Vec<u8> = unavailable_cards.iter()
            .map(|card| card.card_face.value())
            .collect();
        values.sort();
        values.dedup();

        // Check for potential straights (4 consecutive cards)
        let mut straight_probability = 0.0;

        // Check for 4 consecutive values
        for window in values.windows(4) {
            if window[3] - window[0] == 3 {
                // We have 4 consecutive values, need one more card to complete the straight

                // Check if we need a card before or after the sequence
                let need_before = window[0] > 2; // Can add a card before if not starting with 2
                let need_after = window[3] < 13; // Can add a card after if not ending with King

                let mut needed_values = Vec::new();
                if need_before {
                    needed_values.push(window[0] - 1);
                }
                if need_after {
                    needed_values.push(window[3] + 1);
                }

                // Special case for A-2-3-4 (need 5)
                if window[0] == 2 && window[3] == 5 && values.contains(&11) { // 11 is Ace
                    needed_values.push(6);
                }

                // Special case for 2-3-4-5 (need A)
                if window[0] == 2 && window[3] == 5 && !values.contains(&11) {
                    needed_values.push(11); // 11 is Ace
                }

                // Count how many of the needed cards are available
                let matching_cards = available_cards.iter()
                    .filter(|card| needed_values.contains(&card.card_face.value()))
                    .count() as f32;

                // Probability of drawing at least one of these cards
                if !available_cards.is_empty() {
                    straight_probability += matching_cards / available_cards.len() as f32;
                }
            }
        }

        // Check for 3 consecutive values with a gap
        for i in 0..values.len().saturating_sub(4) {
            if values[i+3] - values[i] == 4 {
                // We have 3 consecutive values with a gap, need the missing value
                let missing_value = values[i] + (values[i+3] - values[i]) / 2;

                // Count how many of the needed cards are available
                let matching_cards = available_cards.iter()
                    .filter(|card| card.card_face.value() == missing_value)
                    .count() as f32;

                // Probability of drawing this card
                if !available_cards.is_empty() {
                    straight_probability += matching_cards / available_cards.len() as f32;
                }
            }
        }

        Ok(straight_probability.max(0.0).min(1.0))
    }

    fn calculate_flush_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate flush"));
        }

        // Get the cards that are already in hand or discarded
        let mut unavailable_cards = self.clone();
        if let Some(discarded) = discarded_cards {
            unavailable_cards.extend(discarded);
        }

        // Check if we already have a flush
        if has_flush(&unavailable_cards) {
            return Ok(1.0);
        }

        // Check if we already have a better hand than flush
        if has_full_house(&unavailable_cards) || has_four_of_a_kind(&unavailable_cards) || 
           has_straight_flush(&unavailable_cards) || has_royal_flush(&unavailable_cards) {
            return Ok(0.0);
        }

        // Calculate probability of getting a flush
        let available_cards: Vec<CardData> = DECK.iter()
            .filter(|card| !unavailable_cards.contains(card))
            .cloned()
            .collect();

        // If we have no cards left to draw, return 0
        if available_cards.is_empty() {
            return Ok(0.0);
        }

        // Count cards by suit
        let suit_counts = count_suits(&unavailable_cards);

        // Calculate probability of completing a flush
        let mut flush_probability = 0.0;
        for (&suit, &count) in &suit_counts {
            if count >= 4 {
                // We need one more card of this suit
                let needed_cards = available_cards.iter()
                    .filter(|card| card.card_suit == suit)
                    .count() as f32;

                // Probability of drawing at least one of these cards
                if !available_cards.is_empty() {
                    flush_probability += needed_cards / available_cards.len() as f32;
                }
            } else if count == 3 {
                // We need two more cards of this suit
                let needed_cards = available_cards.iter()
                    .filter(|card| card.card_suit == suit)
                    .count() as f32;

                // Probability of drawing at least two of these cards
                // This is a simplified calculation
                if available_cards.len() >= 2 && needed_cards >= 2.0 {
                    flush_probability += (needed_cards / available_cards.len() as f32) * 0.3;
                }
            }
        }

        Ok(flush_probability.max(0.0).min(1.0))
    }

    fn calculate_full_house_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate full house"));
        }

        // Get the cards that are already in hand or discarded
        let mut unavailable_cards = self.clone();
        if let Some(discarded) = discarded_cards {
            unavailable_cards.extend(discarded);
        }

        // Check if we already have a full house
        if has_full_house(&unavailable_cards) {
            return Ok(1.0);
        }

        // Check if we already have a better hand than full house
        if has_four_of_a_kind(&unavailable_cards) || has_straight_flush(&unavailable_cards) || 
           has_royal_flush(&unavailable_cards) {
            return Ok(0.0);
        }

        // Calculate probability of getting a full house
        let available_cards: Vec<CardData> = DECK.iter()
            .filter(|card| !unavailable_cards.contains(card))
            .cloned()
            .collect();

        // If we have no cards left to draw, return 0
        if available_cards.is_empty() {
            return Ok(0.0);
        }

        // Count cards by face
        let face_counts = count_faces(&unavailable_cards);

        // Calculate probability of completing a full house
        let mut full_house_probability = 0.0;

        // Check if we have three of a kind
        let mut has_three = false;
        let mut has_pair = false;

        for &count in face_counts.values() {
            if count >= 3 {
                has_three = true;
            }
            if count >= 2 {
                has_pair = true;
            }
        }

        if has_three && has_pair {
            // We already have a full house
            return Ok(1.0);
        } else if has_three {
            // We have three of a kind, need a pair
            // Find faces that we have exactly one of
            let mut single_faces: Vec<CardFace> = Vec::new();
            for (&face, &count) in &face_counts {
                if count == 1 {
                    single_faces.push(face);
                }
            }

            for face in single_faces {
                // Count how many cards of this face are available
                let matching_cards = available_cards.iter()
                    .filter(|card| card.card_face == face)
                    .count() as f32;

                // Probability of drawing at least one of these cards
                if !available_cards.is_empty() {
                    full_house_probability += matching_cards / available_cards.len() as f32;
                }
            }
        } else if has_pair {
            // We have a pair, need three of a kind
            // Find faces that we have exactly two of
            let mut pair_faces: Vec<CardFace> = Vec::new();
            for (&face, &count) in &face_counts {
                if count == 2 {
                    pair_faces.push(face);
                }
            }

            for face in pair_faces {
                // Count how many cards of this face are available
                let matching_cards = available_cards.iter()
                    .filter(|card| card.card_face == face)
                    .count() as f32;

                // Probability of drawing at least one of these cards
                if !available_cards.is_empty() {
                    full_house_probability += matching_cards / available_cards.len() as f32;
                }
            }

            // Check if we have two pairs
            let mut pair_count = 0;
            for (_, &count) in &face_counts {
                if count >= 2 {
                    pair_count += 1;
                }
            }

            if pair_count >= 2 {
                // We have two pairs, need one more card to match one of the pairs
                full_house_probability *= 2.0; // Increase probability since we have two chances
            }
        }

        Ok(full_house_probability.max(0.0).min(1.0))
    }

    fn calculate_four_of_a_kind_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!(
                "Not enough cards to calculate four of a kind"
            ));
        }

        // Get the cards that are already in hand or discarded
        let mut unavailable_cards = self.clone();
        if let Some(discarded) = discarded_cards {
            unavailable_cards.extend(discarded);
        }

        // Check if we already have four of a kind
        if has_four_of_a_kind(&unavailable_cards) {
            return Ok(1.0);
        }

        // Check if we already have a better hand than four of a kind
        if has_straight_flush(&unavailable_cards) || has_royal_flush(&unavailable_cards) {
            return Ok(0.0);
        }

        // Calculate probability of getting four of a kind
        let available_cards: Vec<CardData> = DECK.iter()
            .filter(|card| !unavailable_cards.contains(card))
            .cloned()
            .collect();

        // If we have no cards left to draw, return 0
        if available_cards.is_empty() {
            return Ok(0.0);
        }

        // Count cards by face
        let face_counts = count_faces(&unavailable_cards);

        // Calculate probability of completing four of a kind
        let mut four_of_a_kind_probability = 0.0;

        // Check if we have three of a kind
        for (&face, &count) in &face_counts {
            if count == 3 {
                // We have three of a kind, need one more card of this face
                let matching_cards = available_cards.iter()
                    .filter(|card| card.card_face == face)
                    .count() as f32;

                // Probability of drawing this card
                if !available_cards.is_empty() {
                    four_of_a_kind_probability += matching_cards / available_cards.len() as f32;
                }
            } else if count == 2 {
                // We have a pair, need two more cards of this face
                // This is much less likely, so we'll use a simplified calculation
                let matching_cards = available_cards.iter()
                    .filter(|card| card.card_face == face)
                    .count() as f32;

                // Probability of drawing both remaining cards
                if available_cards.len() >= 2 && matching_cards >= 2.0 {
                    four_of_a_kind_probability += (matching_cards / available_cards.len() as f32) * 0.1;
                }
            }
        }

        Ok(four_of_a_kind_probability.max(0.0).min(1.0))
    }

    fn calculate_straight_flush_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!(
                "Not enough cards to calculate straight flush"
            ));
        }

        // Get the cards that are already in hand or discarded
        let mut unavailable_cards = self.clone();
        if let Some(discarded) = discarded_cards {
            unavailable_cards.extend(discarded);
        }

        // Check if we already have a straight flush
        if has_straight_flush(&unavailable_cards) {
            return Ok(1.0);
        }

        // Check if we already have a better hand than straight flush
        if has_royal_flush(&unavailable_cards) {
            return Ok(0.0);
        }

        // Calculate probability of getting a straight flush
        let available_cards: Vec<CardData> = DECK.iter()
            .filter(|card| !unavailable_cards.contains(card))
            .cloned()
            .collect();

        // If we have no cards left to draw, return 0
        if available_cards.is_empty() {
            return Ok(0.0);
        }

        // Group cards by suit
        let mut cards_by_suit: HashMap<CardSuit, Vec<CardData>> = HashMap::new();
        for card in &unavailable_cards {
            cards_by_suit.entry(card.card_suit).or_insert_with(Vec::new).push(*card);
        }

        // Calculate probability of completing a straight flush
        let mut straight_flush_probability = 0.0;

        // For each suit, check if we're close to a straight flush
        for (suit, suit_cards) in &cards_by_suit {
            if suit_cards.len() >= 4 {
                // Get unique face values for this suit
                let mut values: Vec<u8> = suit_cards.iter()
                    .map(|card| card.card_face.value())
                    .collect();
                values.sort();
                values.dedup();

                // Check for 4 consecutive values
                for window in values.windows(4) {
                    if window[3] - window[0] == 3 {
                        // We have 4 consecutive values of the same suit, need one more card

                        // Check if we need a card before or after the sequence
                        let need_before = window[0] > 2; // Can add a card before if not starting with 2
                        let need_after = window[3] < 13; // Can add a card after if not ending with King

                        let mut needed_values = Vec::new();
                        if need_before {
                            needed_values.push(window[0] - 1);
                        }
                        if need_after {
                            needed_values.push(window[3] + 1);
                        }

                        // Special case for A-2-3-4 (need 5)
                        if window[0] == 2 && window[3] == 5 && values.contains(&11) { // 11 is Ace
                            needed_values.push(6);
                        }

                        // Special case for 2-3-4-5 (need A)
                        if window[0] == 2 && window[3] == 5 && !values.contains(&11) {
                            needed_values.push(11); // 11 is Ace
                        }

                        // Count how many of the needed cards of this suit are available
                        let matching_cards = available_cards.iter()
                            .filter(|card| card.card_suit == *suit && needed_values.contains(&card.card_face.value()))
                            .count() as f32;

                        // Probability of drawing at least one of these cards
                        if !available_cards.is_empty() {
                            straight_flush_probability += matching_cards / available_cards.len() as f32;
                        }
                    }
                }
            }
        }

        Ok(straight_flush_probability.max(0.0).min(1.0))
    }

    fn calculate_royal_flush_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate royal flush"));
        }

        // Get the cards that are already in hand or discarded
        let mut unavailable_cards = self.clone();
        if let Some(discarded) = discarded_cards {
            unavailable_cards.extend(discarded);
        }

        // Check if we already have a royal flush
        if has_royal_flush(&unavailable_cards) {
            return Ok(1.0);
        }

        // Calculate probability of getting a royal flush
        let available_cards: Vec<CardData> = DECK.iter()
            .filter(|card| !unavailable_cards.contains(card))
            .cloned()
            .collect();

        // If we have no cards left to draw, return 0
        if available_cards.is_empty() {
            return Ok(0.0);
        }

        // Group cards by suit
        let mut cards_by_suit: HashMap<CardSuit, Vec<CardData>> = HashMap::new();
        for card in &unavailable_cards {
            cards_by_suit.entry(card.card_suit).or_insert_with(Vec::new).push(*card);
        }

        // Calculate probability of completing a royal flush
        let mut royal_flush_probability = 0.0;

        // For each suit, check if we're close to a royal flush
        for (suit, suit_cards) in &cards_by_suit {
            if suit_cards.len() >= 4 {
                // Check if we have the royal flush cards (10, J, Q, K, A of same suit)
                let has_ten = suit_cards.iter().any(|card| card.card_face == CardFace::Ten);
                let has_jack = suit_cards.iter().any(|card| card.card_face == CardFace::Jack);
                let has_queen = suit_cards.iter().any(|card| card.card_face == CardFace::Queen);
                let has_king = suit_cards.iter().any(|card| card.card_face == CardFace::King);
                let has_ace = suit_cards.iter().any(|card| card.card_face == CardFace::Ace);

                // Count how many of the royal flush cards we have
                let royal_count = [has_ten, has_jack, has_queen, has_king, has_ace].iter().filter(|&&has| has).count();

                if royal_count == 4 {
                    // We have 4 of the 5 cards needed for a royal flush
                    // Determine which card we're missing
                    let mut needed_faces = Vec::new();
                    if !has_ten { needed_faces.push(CardFace::Ten); }
                    if !has_jack { needed_faces.push(CardFace::Jack); }
                    if !has_queen { needed_faces.push(CardFace::Queen); }
                    if !has_king { needed_faces.push(CardFace::King); }
                    if !has_ace { needed_faces.push(CardFace::Ace); }

                    // Count how many of the needed cards of this suit are available
                    let matching_cards = available_cards.iter()
                        .filter(|card| card.card_suit == *suit && needed_faces.contains(&card.card_face))
                        .count() as f32;

                    // Probability of drawing the needed card
                    if !available_cards.is_empty() {
                        royal_flush_probability += matching_cards / available_cards.len() as f32;
                    }
                }
            }
        }

        Ok(royal_flush_probability.max(0.0).min(1.0))
    }
}
