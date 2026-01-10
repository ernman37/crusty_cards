use rand::rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::ops::{Sub, SubAssign, Add, AddAssign, Mul, MulAssign};
use std::str::FromStr;
use std::{fmt};
use std::convert::{From, TryFrom};

use crate::Card;
use crate::CardComparator;
use crate::DeckFactory;

/// A struct representing a deck of playing cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string_delimiter(' '))
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

    /// Returns an iterator over the cards in the deck.
    pub fn iter<'a>(&'a self) -> std::collections::vec_deque::Iter<'a, Card> {
        self.cards.iter()
    }

    /// Returns a mutable iterator over the cards in the deck.
    pub fn iter_mut<'a>(&'a mut self) -> std::collections::vec_deque::IterMut<'a, Card> {
        self.cards.iter_mut()
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

    /// Returns a string representation of the deck with the specified delimiter.
    pub fn as_string_delimiter(&self, delimiter: char) -> String {
        self.cards
            .iter()
            .map(|card| format!("{}", card))
            .collect::<Vec<String>>()
            .join(&delimiter.to_string())
    }

    /// Creates a Deck from a string representation of cards separated by a delimiter.
    pub fn from_str_delimiter(s: &str, delimiter: char) -> Result<Self, String> {
        let mut cards = VecDeque::new();
        for part in s.split(delimiter) {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            match Card::from_str(part) {
                Ok(card) => cards.push_back(card),
                Err(e) => return Err(format!("Failed to parse card '{}': {}", part, e)),
            }
        }
        Ok(Deck::new(cards))
    }

    /// Serializes the deck to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serializes the deck to a pretty-printed JSON string.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Creates a Deck from a JSON string.
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    /// Serializes the deck to a YAML string.
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    /// Creates a Deck from a YAML string.
    pub fn from_yaml(s: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(s)
    }

    /// Sorts the deck using a custom comparator.
    ///
    /// # Example
    /// ```
    /// use crusty_cards::{Deck, Standard52, StandardComparator, AceLowComparator};
    ///
    /// let mut deck = Deck::from_factory(Standard52);
    /// deck.shuffle();
    ///
    /// // Sort with Ace high (standard)
    /// deck.sort_by_comparator(&StandardComparator);
    ///
    /// // Or sort with Ace low
    /// deck.sort_by_comparator(&AceLowComparator);
    /// ```
    pub fn sort_by_comparator<C: CardComparator>(&mut self, comparator: &C) {
        let mut cards_vec: Vec<Card> = self.cards.drain(..).collect();
        cards_vec.sort_by(|a, b| comparator.compare(a, b));
        self.cards = VecDeque::from(cards_vec);
    }

    /// Sorts the deck using a custom comparison function.
    ///
    /// # Example
    /// ```
    /// use crusty_cards::{Deck, Standard52};
    ///
    /// let mut deck = Deck::from_factory(Standard52);
    /// deck.shuffle();
    ///
    /// // Sort by rank value descending
    /// deck.sort_by(|a, b| b.rank().value().cmp(&a.rank().value()));
    /// ```
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Card, &Card) -> std::cmp::Ordering,
    {
        let mut cards_vec: Vec<Card> = self.cards.drain(..).collect();
        cards_vec.sort_by(compare);
        self.cards = VecDeque::from(cards_vec);
    }

    pub fn as_csv(&self) -> String {
        let mut csv = "Rank,Suit\n".to_string();
        for card in &self.cards {
            csv.push_str(&card.as_csv_row());
            csv.push('\n');
        }
        csv
    }

    pub fn from_csv(s: &str) -> Result<Self, String> {
        let mut cards = VecDeque::new();
        for (i, line) in s.lines().enumerate() {
            if i == 0 {
                continue; // Skip header
            }
            if line.trim().is_empty() {
                continue; // Skip empty lines
            }
            match Card::from_csv_row(line) {
                Ok(card) => cards.push_back(card),
                Err(e) => return Err(format!("Failed to parse line {}: {}", i + 1, e)),
            }
        }
        Ok(Deck::new(cards))
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

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = std::collections::vec_deque::IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'a> IntoIterator for &'a Deck {
    type Item = &'a Card;
    type IntoIter = std::collections::vec_deque::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Deck {
    type Item = &'a mut Card;
    type IntoIter = std::collections::vec_deque::IterMut<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl FromStr for Deck {
    type Err = String;

    /// Creates a Deck from a string representation of cards separated by spaces.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Deck::from_str_delimiter(s, ' ')
    }
}
