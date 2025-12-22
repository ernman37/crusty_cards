mod card;
pub use card::{Card, Color, Suit, Rank};

use std::collections::VecDeque;
use std::ops::{Add, Sub};

/// A struct representing a deck of playing cards.
pub struct Deck {
    cards: VecDeque<Card>,
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
impl Deck {
    /// Creates a new Deck with the given cards.
    pub fn new(cards: VecDeque<Card>) -> Self {
        Deck { cards }
    }

    /// Cuts the deck at the given index.
    pub fn cut(&mut self, index: usize) {
        if index >= self.cards.len() {
            return;
        }
        let mut top = self.cards.split_off(index);
        top.append(&mut self.cards);
        self.cards = top;
    }

    /// Returns the number of cards in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Checks if the deck is empty.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Displays the cards in the deck as a vector of strings.
    pub fn display(&self) -> Vec<String> {
        self.cards.iter().map(|card| card.display()).collect()
    }

    /// Shuffles the deck of cards.
    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Deals a card from the top of the deck.
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    /// Deals a card from the bottom of the deck.
    pub fn deal_bottom(&mut self) -> Option<Card> {
        self.cards.pop_back()
    }

    /// Adds a card to the top of the deck.
    pub fn add_card(&mut self, card: Card) {
        self.cards.push_front(card);
    }

    /// Adds a card to the bottom of the deck.
    pub fn add_card_bottom(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    /// Peeks at the top card of the deck without removing it.
    pub fn peek(&self) -> Option<&Card> {
        self.cards.front()
    }

    /// Peeks at the bottom card of the deck without removing it.
    pub fn peek_bottom(&self) -> Option<&Card> {
        self.cards.back()
    }

    /// Clears the deck of all cards.
    pub fn clear(&mut self) {
        self.cards.clear();
    }
}
