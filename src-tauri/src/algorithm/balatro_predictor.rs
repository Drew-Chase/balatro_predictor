use crate::algorithm::card_data::{BalatroPredictor, CardData};

impl BalatroPredictor for Vec<CardData> {
    fn calculate_high_card_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate high card"));
        }
        

        Ok(0f32)
    }

    fn calculate_pair_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate pair"));
        }

        Ok(0f32)
    }

    fn calculate_two_pair_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate two pair"));
        }

        Ok(0f32)
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

        Ok(0f32)
    }

    fn calculate_straight_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate straight"));
        }

        Ok(0f32)
    }

    fn calculate_flush_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate flush"));
        }

        Ok(0f32)
    }

    fn calculate_full_house_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate full house"));
        }

        Ok(0f32)
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

        Ok(0f32)
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

        Ok(0f32)
    }

    fn calculate_royal_flush_probability(
        &mut self,
        discarded_cards: Option<Vec<CardData>>,
    ) -> anyhow::Result<f32> {
        if self.len() < 5 {
            return Err(anyhow::anyhow!("Not enough cards to calculate royal flush"));
        }

        Ok(0f32)
    }
}
