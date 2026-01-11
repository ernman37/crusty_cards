use crate::{Card, DeckFactory, Rank, Suit};

use std::collections::VecDeque;

/// Factory for a standard 52-card deck.
///
/// Generates all combinations of 4 suits × 13 ranks (Two through Ace).
/// Cards are ordered by rank, then by suit within each rank.
///
/// # Examples
///
/// ```rust
/// use crusty_cards::{Deck, Standard52};
///
/// let deck = Deck::from_factory(Standard52);
/// assert_eq!(deck.len(), 52);
/// ```
pub struct Standard52;

impl DeckFactory for Standard52 {
    fn generate(&self) -> VecDeque<Card> {
        Rank::STANDARD
            .iter()
            .flat_map(|&rank| Suit::ALL.iter().map(move |&suit| Card::new(suit, rank)))
            .collect()
    }
}

/// Factory for a 54-card deck with 2 jokers.
///
/// Generates a standard 52-card deck plus a red joker (Hearts) and black joker (Spades).
///
/// # Examples
///
/// ```rust
/// use crusty_cards::{Deck, Standard54};
///
/// let deck = Deck::from_factory(Standard54);
/// assert_eq!(deck.len(), 54);
/// ```
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
