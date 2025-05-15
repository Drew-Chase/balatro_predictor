mod test {
    #[test]
    fn test_deck_builder() {
        let deck = balatro_predictor_lib::algorithm::deck::DECK;
        println!("Deck: {:?}", deck);
        assert_eq!(deck.len(), 52);
    }
}
