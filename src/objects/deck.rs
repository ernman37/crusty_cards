use rand::rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::ops::{Sub, SubAssign, Add, AddAssign, Mul, MulAssign};
use std::fmt;
use std::convert::{From, TryFrom};

use crate::Card;
use crate::DeckFactory;

/// A struct representing a deck of playing cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in &self.cards {
            write!(f, "{} ", card)?;
        }
        Ok(())
    }
}

impl Deck {
    /// Creates a new Deck with the given cards.
    pub fn new(cards: VecDeque<Card>) -> Self {
        Deck { cards }
    }

    /// Creates a new Deck using a DeckFactory.
    pub fn from_factory<F>(factory: F) -> Self
    where
        F: DeckFactory,
    {
        Self {
            cards: factory.generate(),
        }
    }

    /// Cuts the deck at the given index.
    pub fn cut(&mut self, index: usize) -> bool {
        if index >= self.cards.len() {
            return false;
        }
        let mut top = self.cards.split_off(index);
        top.append(&mut self.cards);
        self.cards = top;
        true
    }

    /// Returns the number of cards in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Checks if the deck is empty.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Shuffles the deck of cards.
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        let mut cards_vec: Vec<Card> = self.cards.drain(..).collect();
        cards_vec.shuffle(&mut rng);
        self.cards = VecDeque::from(cards_vec);
    }

    /// Shuffle Times
    pub fn shuffle_times(&mut self, times: usize) {
        for _ in 0..times {
            self.shuffle();
        }
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

impl Add<Card> for Deck {
    type Output = Deck;

    /// Adds specified Card to the deck (top), returns a new Deck.
    fn add(self, rhs: Card) -> Deck {
        let mut new_deck = self.clone();
        new_deck.add_card(rhs);
        new_deck
    }
}

impl AddAssign<Card> for Deck {
    /// Adds specified Card to the deck (top) in place.
    fn add_assign(&mut self, rhs: Card) {
        self.add_card(rhs);
    }
}

impl Sub<Card> for Deck {
    type Output = Deck;

    /// Removes specified Card from the deck, returns a new Deck.
    fn sub(self, rhs: Card) -> Deck {
        let mut new_deck = self.clone();
        new_deck.cards.retain(|c| c != &rhs);
        new_deck
    }
}

impl SubAssign<Card> for Deck {
    /// Removes specified Card from the deck in place.
    fn sub_assign(&mut self, rhs: Card) {
        *self = self.clone() - rhs;
    }
}

impl Mul<usize> for Deck {
    type Output = Deck;

    /// Multiplies the deck by duplicating its cards n times, returns a new Deck.
    fn mul(self, rhs: usize) -> Deck {
        let mut new_deck = Deck::new(VecDeque::new());
        for _ in 0..rhs {
            for card in &self.cards {
                new_deck.add_card(*card);
            }
        }
        new_deck
    }
}

impl MulAssign<usize> for Deck {
    /// Multiplies the deck by duplicating its cards n times in place.
    fn mul_assign(&mut self, rhs: usize) {
        let original_deck = self.clone();
        self.clear();
        for _ in 0..rhs {
            for card in &original_deck.cards {
                self.add_card(*card);
            }
        }
    }
}

impl Add<Deck> for Deck {
    type Output = Deck;

    /// Adds cards from rhs Deck to self Deck, returns a new Deck.
    fn add(self, rhs: Deck) -> Deck {
        let mut new_deck = self.clone();
        for card in rhs.cards {
            new_deck.add_card(card);
        }
        new_deck
    }
}

impl AddAssign<Deck> for Deck {
    /// Adds cards from rhs Deck to self Deck in place.
    fn add_assign(&mut self, rhs: Deck) {
        for card in rhs.cards {
            self.add_card(card);
        }
    }
}

impl Sub<Deck> for Deck {
    type Output = Deck;

    /// Removes cards from rhs Deck from self Deck, returns a new Deck.
    fn sub(self, rhs: Deck) -> Deck {
        let mut new_deck = self.clone();
        for card in rhs.cards {
            new_deck = new_deck.clone() - card;
        }
        new_deck
    }
}

impl SubAssign<Deck> for Deck {
    /// Removes cards from rhs Deck from self Deck in place.
    fn sub_assign(&mut self, rhs: Deck) {
        *self = self.clone() - rhs;
    }
}

impl TryFrom<Vec<usize>> for Deck {
    type Error = String;

    /// Tries to create a Deck from a vector of usize indices.
    fn try_from(values: Vec<usize>) -> Result<Self, Self::Error> {
        let mut cards = VecDeque::new();
        for value in values {
            if let Ok(card) = Card::try_from(value) {
                cards.push_back(card);
            } else {
                return Err(format!("Invalid card value: {}", value));
            }
        }
        Ok(Deck::new(cards))
    }
}

impl From<Deck> for Vec<usize> {
    /// Converts a Deck into a vector of usize indices.
    fn from(deck: Deck) -> Self {
        deck.cards.iter().map(|card| usize::from(*card)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Deck;
    use crate::{Card, Rank, Suit};
    use std::collections::VecDeque;

    #[test]
    fn test_deck_creation() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let deck = Deck::new(cards.clone());
        assert_eq!(deck.len(), 2);
        assert_eq!(deck.peek(), Some(&cards[0]));
    }

    #[test]
    fn test_deck_cut() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Clubs, Rank::Jack),
        ]);
        let mut deck = Deck::new(cards);
        let result = deck.cut(2);
        assert_eq!(deck.peek(), Some(&Card::new(Suit::Diamonds, Rank::Queen)));
        assert!(result);

        let result = deck.cut(10); // Cutting beyond length should do nothing
        assert!(!result);
        assert_eq!(deck.peek(), Some(&Card::new(Suit::Diamonds, Rank::Queen)));
    }

    #[test]
    fn test_deck_is_empty() {
        let cards = VecDeque::new();
        let mut deck = Deck::new(cards);
        assert!(deck.is_empty());
        deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
        assert!(!deck.is_empty());
    }

    #[test]
    fn test_deck_display() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let deck = Deck::new(cards);
        let display = format!("{}", deck);
        let card1 = Card::new(Suit::Hearts, Rank::Ace);
        let card2 = Card::new(Suit::Spades, Rank::King);
        assert_eq!(display, format!("{} {} ", card1, card2));
    }

    #[test]
    fn test_deck_shuffle() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Clubs, Rank::Jack),
        ]);
        let mut deck = Deck::new(cards);
        let original_order = format!("{}", deck);
        // Try shuffling multiple times to ensure order changes (very low chance of false negative)
        for _ in 0..100 {
            deck.shuffle_times(1);
            let shuffled_order = format!("{}", deck);
            if original_order != shuffled_order {
                return;
            }
        }
        assert!(
            false,
            "Deck shuffle did not change order after multiple attempts"
        );
    }

    #[test]
    fn test_deck_deal() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let mut deck = Deck::new(cards);
        let dealt_card = deck.deal();
        assert_eq!(dealt_card, Some(Card::new(Suit::Hearts, Rank::Ace)));
        assert_eq!(deck.len(), 1);
        let dealt_card2 = deck.deal();
        assert_eq!(dealt_card2, Some(Card::new(Suit::Spades, Rank::King)));
        assert_eq!(deck.len(), 0);
        let dealt_card3 = deck.deal();
        assert_eq!(dealt_card3, None);
    }

    #[test]
    fn test_deck_deal_bottom() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let mut deck = Deck::new(cards);
        let dealt_card = deck.deal_bottom();
        assert_eq!(dealt_card, Some(Card::new(Suit::Spades, Rank::King)));
        assert_eq!(deck.len(), 1);
        let dealt_card2 = deck.deal_bottom();
        assert_eq!(dealt_card2, Some(Card::new(Suit::Hearts, Rank::Ace)));
        assert_eq!(deck.len(), 0);
        let dealt_card3 = deck.deal_bottom();
        assert_eq!(dealt_card3, None);
    }

    #[test]
    fn test_deck_add_card() {
        let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
        let mut deck = Deck::new(cards);
        deck.add_card(Card::new(Suit::Spades, Rank::King));
        assert_eq!(deck.len(), 2);
        assert_eq!(deck.peek(), Some(&Card::new(Suit::Spades, Rank::King)));
    }

    #[test]
    fn test_deck_add_card_bottom() {
        let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
        let mut deck = Deck::new(cards);
        deck.add_card_bottom(Card::new(Suit::Spades, Rank::King));
        assert_eq!(deck.len(), 2);
        assert_eq!(
            deck.peek_bottom(),
            Some(&Card::new(Suit::Spades, Rank::King))
        );
    }

    #[test]
    fn test_deck_peek() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let mut deck = Deck::new(cards);
        let top_card = deck.peek();
        assert_eq!(top_card, Some(&Card::new(Suit::Hearts, Rank::Ace)));
        assert_eq!(deck.len(), 2);
        deck.clear();
        let empty_peek = deck.peek();
        assert_eq!(empty_peek, None);
    }

    #[test]
    fn test_deck_peek_bottom() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let mut deck = Deck::new(cards);
        let bottom_card = deck.peek_bottom();
        assert_eq!(bottom_card, Some(&Card::new(Suit::Spades, Rank::King)));
        assert_eq!(deck.len(), 2);
        deck.clear();
        let empty_peek = deck.peek_bottom();
        assert_eq!(empty_peek, None);
    }

    #[test]
    fn test_deck_clear() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let mut deck = Deck::new(cards);
        deck.clear();
        assert_eq!(deck.len(), 0);
    }

    #[test]
    fn test_deck_serialization() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let deck = Deck::new(cards);
        let serialized = serde_json::to_string(&deck).unwrap();
        let deserialized: Deck = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deck.len(), deserialized.len());
        assert_eq!(deck.to_string(), deserialized.to_string());
    }

    #[test]
    fn test_deck_add_card_operator() {
        let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
        let deck = Deck::new(cards);
        let new_deck = deck + Card::new(Suit::Spades, Rank::King);
        assert_eq!(new_deck.len(), 2);
        assert_eq!(new_deck.peek(), Some(&Card::new(Suit::Spades, Rank::King)));
    }

    #[test]
    fn test_deck_add_card_assign_operator() {
        let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
        let mut deck = Deck::new(cards);
        deck += Card::new(Suit::Spades, Rank::King);
        assert_eq!(deck.len(), 2);
        assert_eq!(deck.peek(), Some(&Card::new(Suit::Spades, Rank::King)));
    }

    #[test]
    fn test_deck_sub_card() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
        ]);
        let deck = Deck::new(cards);
        let subtracted_deck = deck - Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(subtracted_deck.len(), 2);
        assert_eq!(
            subtracted_deck.peek(),
            Some(&Card::new(Suit::Spades, Rank::King))
        );
    }

    #[test]
    fn test_deck_sub_assign_card() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
        ]);
        let mut deck = Deck::new(cards);
        deck -= Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(deck.len(), 2);
        assert_eq!(
            deck.peek(),
            Some(&Card::new(Suit::Spades, Rank::King))
        );
    }

    #[test]
    fn test_add_deck_deck() {
        let cards1 = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let cards2 = VecDeque::from(vec![
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Clubs, Rank::Jack),
        ]);
        let deck1 = Deck::new(cards1);
        let deck2 = Deck::new(cards2);
        let combined_deck = deck1 + deck2;
        assert_eq!(combined_deck.len(), 4);
    }

    #[test]
    fn test_add_assign_deck_deck() {
        let cards1 = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let cards2 = VecDeque::from(vec![
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Clubs, Rank::Jack),
        ]);
        let mut deck1 = Deck::new(cards1);
        let deck2 = Deck::new(cards2);
        deck1 += deck2;
        assert_eq!(deck1.len(), 4);
    }

    #[test]
    fn test_deck_sub_deck() {
        let cards1 = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
        ]);
        let cards2 = VecDeque::from(vec![
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Clubs, Rank::Jack),
        ]);
        let deck1 = Deck::new(cards1);
        let deck2 = Deck::new(cards2);
        let subtracted_deck = deck1 - deck2;
        assert_eq!(subtracted_deck.len(), 2);
        assert_eq!(
            subtracted_deck.peek(),
            Some(&Card::new(Suit::Hearts, Rank::Ace))
        );
    }

    #[test]
    fn test_deck_sub_assign_deck() {
        let cards1 = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
        ]);
        let cards2 = VecDeque::from(vec![
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Clubs, Rank::Jack),
        ]);
        let mut deck1 = Deck::new(cards1);
        let deck2 = Deck::new(cards2);
        deck1 -= deck2;
        assert_eq!(deck1.len(), 2);
        assert_eq!(
            deck1.peek(),
            Some(&Card::new(Suit::Hearts, Rank::Ace))
        );
    }

    #[test]
    fn test_deck_mul() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let deck = Deck::new(cards);
        let multiplied_deck = deck * 3;
        assert_eq!(multiplied_deck.len(), 6);
    }

    #[test]
    fn test_deck_mul_assign() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let mut deck = Deck::new(cards);
        deck *= 3;
        assert_eq!(deck.len(), 6);
    }

    #[test]
    fn test_deck_try_from_vec_usize() {
        let values = vec![0, 12, 25]; // Ace of Hearts, King of Hearts, King of Diamonds
        let deck = Deck::try_from(values).unwrap();
        assert_eq!(deck.len(), 3);
    }

    #[test]
    fn test_deck_from_deck_to_vec_usize() {
        let cards = VecDeque::from(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ]);
        let deck = Deck::new(cards);
        let values: Vec<usize> = Vec::from(deck);
        assert_eq!(values, vec![12, 53]);

    }
}
