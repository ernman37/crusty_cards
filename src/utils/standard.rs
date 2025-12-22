
use crate::{DeckFactory, Card, Suit, Rank};

use std::collections::VecDeque;

/// A standard 52-card deck factory
pub struct Standard52;

impl DeckFactory for Standard52 {
    fn generate(&self) -> VecDeque<Card> {
        let suits = vec![
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Clubs,
            Suit::Spades
        ];
        let ranks = vec![
            Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
            Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King, Rank::Ace
        ];

        ranks.iter().flat_map(|&rank| {
            suits.iter().map(move |&suit| Card::new(suit, rank))
        }).collect()
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
    use crate::{ Rank };
    use crate::DeckFactory;

    #[test]
    fn test_standard_52_deck() {
        let deck = Standard52.generate();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn test_standard_54_deck() {
        let deck = Standard54.generate();
        assert_eq!(deck.len(), 54);
        let joker_count = deck.iter().filter(|&card| *card.rank() == Rank::Joker).count();
        assert_eq!(joker_count, 2);
    }
}
