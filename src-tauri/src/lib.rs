use crate::algorithm::card_data::{BalatroPredictor, CardData};

pub mod algorithm;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            calculate_high_card_probability,
            calculate_pair_probability,
            calculate_two_pair_probability,
            calculate_three_of_a_kind_probability,
            calculate_straight_probability,
            calculate_flush_probability,
            calculate_full_house_probability,
            calculate_four_of_a_kind_probability,
            calculate_straight_flush_probability,
            calculate_royal_flush_probability
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn calculate_high_card_probability(
    hand: Vec<CardData>,
    discarded_cards: Vec<CardData>,
) -> Result<f32, String> {
    let mut hand = hand.to_vec().clone();
    let probability = hand
        .calculate_high_card_probability(if discarded_cards.is_empty() {
            None
        } else {
            Some(discarded_cards)
        })
        .map_err(|e| e.to_string())?;
    Ok(probability)
}

#[tauri::command]
fn calculate_pair_probability(
    hand: Vec<CardData>,
    discarded_cards: Vec<CardData>,
) -> Result<f32, String> {
    let mut hand = hand.to_vec().clone();
    let probability = hand
        .calculate_pair_probability(if discarded_cards.is_empty() {
            None
        } else {
            Some(discarded_cards)
        })
        .map_err(|e| e.to_string())?;
    Ok(probability)
}

#[tauri::command]
fn calculate_two_pair_probability(
    hand: Vec<CardData>,
    discarded_cards: Vec<CardData>,
) -> Result<f32, String> {
    let mut hand = hand.to_vec().clone();
    let probability = hand
        .calculate_two_pair_probability(if discarded_cards.is_empty() {
            None
        } else {
            Some(discarded_cards)
        })
        .map_err(|e| e.to_string())?;
    Ok(probability)
}

#[tauri::command]
fn calculate_three_of_a_kind_probability(
    hand: Vec<CardData>,
    discarded_cards: Vec<CardData>,
) -> Result<f32, String> {
    let mut hand = hand.to_vec().clone();
    let probability = hand
        .calculate_three_of_a_kind_probability(if discarded_cards.is_empty() {
            None
        } else {
            Some(discarded_cards)
        })
        .map_err(|e| e.to_string())?;
    Ok(probability)
}

#[tauri::command]
fn calculate_straight_probability(
    hand: Vec<CardData>,
    discarded_cards: Vec<CardData>,
) -> Result<f32, String> {
    let mut hand = hand.to_vec().clone();
    let probability = hand
        .calculate_straight_probability(if discarded_cards.is_empty() {
            None
        } else {
            Some(discarded_cards)
        })
        .map_err(|e| e.to_string())?;
    Ok(probability)
}

#[tauri::command]
fn calculate_flush_probability(
    hand: Vec<CardData>,
    discarded_cards: Vec<CardData>,
) -> Result<f32, String> {
    let mut hand = hand.to_vec().clone();
    let probability = hand
        .calculate_flush_probability(if discarded_cards.is_empty() {
            None
        } else {
            Some(discarded_cards)
        })
        .map_err(|e| e.to_string())?;
    Ok(probability)
}

#[tauri::command]
fn calculate_full_house_probability(
    hand: Vec<CardData>,
    discarded_cards: Vec<CardData>,
) -> Result<f32, String> {
    let mut hand = hand.to_vec().clone();
    let probability = hand
        .calculate_full_house_probability(if discarded_cards.is_empty() {
            None
        } else {
            Some(discarded_cards)
        })
        .map_err(|e| e.to_string())?;
    Ok(probability)
}

#[tauri::command]
fn calculate_four_of_a_kind_probability(
    hand: Vec<CardData>,
    discarded_cards: Vec<CardData>,
) -> Result<f32, String> {
    let mut hand = hand.to_vec().clone();
    let probability = hand
        .calculate_four_of_a_kind_probability(if discarded_cards.is_empty() {
            None
        } else {
            Some(discarded_cards)
        })
        .map_err(|e| e.to_string())?;
    Ok(probability)
}

#[tauri::command]
fn calculate_straight_flush_probability(
    hand: Vec<CardData>,
    discarded_cards: Vec<CardData>,
) -> Result<f32, String> {
    let mut hand = hand.to_vec().clone();
    let probability = hand
        .calculate_straight_flush_probability(if discarded_cards.is_empty() {
            None
        } else {
            Some(discarded_cards)
        })
        .map_err(|e| e.to_string())?;
    Ok(probability)
}

#[tauri::command]
fn calculate_royal_flush_probability(
    hand: Vec<CardData>,
    discarded_cards: Vec<CardData>,
) -> Result<f32, String> {
    let mut hand = hand.to_vec().clone();
    let probability = hand
        .calculate_royal_flush_probability(if discarded_cards.is_empty() {
            None
        } else {
            Some(discarded_cards)
        })
        .map_err(|e| e.to_string())?;
    Ok(probability)
}
