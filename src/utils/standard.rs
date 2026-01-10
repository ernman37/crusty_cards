use crate::{Card, DeckFactory, Rank, Suit};

use std::collections::VecDeque;

/// A standard 52-card deck factory
pub struct Standard52;

impl DeckFactory for Standard52 {
    fn generate(&self) -> VecDeque<Card> {
        Rank::STANDARD
            .iter()
            .flat_map(|&rank| Suit::ALL.iter().map(move |&suit| Card::new(suit, rank)))
            .collect()
    }
}

/// A standard 54-card deck factory (including 2 jokers)
pub struct Standard54;
impl DeckFactory for Standard54 {
    fn generate(&self) -> VecDeque<Card> {
        let mut cards = Standard52.generate();
        let suits = vec![Suit::Hearts, Suit::Spades];
        for &suit in &suits {
            cards.push_back(Card::new(suit, Rank::Joker));
        }
        cards
    }
}

#[cfg(test)]
mod tests {
    use super::{Standard52, Standard54};
    use crate::DeckFactory;
    use crate::Rank;

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
}
