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
