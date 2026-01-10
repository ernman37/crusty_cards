use crusty_cards::{DeckFactory, Rank, Standard52, Standard54};

#[test]
fn test_standard_52_deck() {
    let deck = Standard52.generate();
    assert_eq!(deck.len(), 52);
}

#[test]
fn test_standard_54_deck() {
    let deck = Standard54.generate();
    assert_eq!(deck.len(), 54);
    let joker_count = deck
        .iter()
        .filter(|&card| card.rank() == Rank::Joker)
        .count();
    assert_eq!(joker_count, 2);
}
