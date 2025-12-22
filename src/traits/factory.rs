use crate::Card;
use std::collections::VecDeque;

/// Trait for generating the cards used within a Deck
pub trait DeckFactory {
    fn generate(&self) -> VecDeque<Card>;
}
