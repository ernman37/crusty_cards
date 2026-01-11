use rand::rng;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::convert::{From, TryFrom};
use std::fmt;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use std::str::FromStr;

use crate::Card;
use crate::CardComparator;
use crate::DeckFactory;

/// A struct representing a deck of playing cards.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str_delimiter(' '))
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
        if index >= self.len() {
            return false;
        }
        let mut top = self.cards.split_off(index);
        top.append(&mut self.cards);
        self.cards = top;
        true
    }

    /// Returns if the card is in the deck.
    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    /// Returns the index of the card in the deck, if it exists.
    pub fn find(&self, card: &Card) -> Option<usize> {
        self.cards.iter().position(|c| c == card)
    }

    /// Inserts a card at the specified index in the deck.
    pub fn insert_at(&mut self, card: Card, index: usize) -> bool {
        if index > self.cards.len() {
            return false;
        }
        self.cards.insert(index, card);
        true
    }

    /// Removes a card from the specified index in the deck
    pub fn remove_at(&mut self, index: usize) -> Option<Card> {
        if index >= self.cards.len() {
            return None;
        }
        self.cards.remove(index)
    }

    /// Reverses the order of cards in self
    pub fn reverse(&mut self) {
        self.cards = self.cards.iter().cloned().rev().collect();
    }

    /// Counts occurrence of a card in deck
    pub fn count(&self, card: &Card) -> usize {
        self.cards.iter().filter(|&&c| c == *card).count()
    }

    /// Returns the number of cards in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns two decks split at the given index.
    /// The first deck contains cards [0, index), the second contains [index, len).
    /// If index > len, returns (clone of self, empty deck).
    pub fn split_at(&self, index: usize) -> (Self, Self) {
        let index = index.min(self.cards.len());
        let left: VecDeque<Card> = self.cards.iter().take(index).cloned().collect();
        let right: VecDeque<Card> = self.cards.iter().skip(index).cloned().collect();
        (Deck::new(left), Deck::new(right))
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

    /// Riffle shuffles the deck of cards.
    pub fn riffle_shuffle(&mut self) {
        let middle = self.len() / 2;
        let mut shuffled = VecDeque::with_capacity(self.len());

        for i in 0..middle {
            shuffled.push_back(self.cards[middle + i]);
            shuffled.push_back(self.cards[i]);
        }

        if self.len() % 2 == 1 {
            shuffled.push_back(self.cards[self.len() - 1]);
        }

        self.cards = shuffled;
    }

    /// Riffle shuffle times
    pub fn riffle_shuffle_times(&mut self, times: usize) {
        for _ in 0..times {
            self.riffle_shuffle();
        }
    }

    /// Overhand Shuffle
    pub fn overhand_shuffle(&mut self) {
        let mut left = Deck::default();
        while !self.is_empty() {
            let random_size = if self.len() == 1 {
                1
            } else {
                rand::rng().random_range(1..=self.len() / 2)
            };
            left.add_cards(self.deal_n(random_size).unwrap());
        }
        self.cards = left.cards;
    }

    /// Overhand shuffle times
    pub fn overhand_shuffle_times(&mut self, times: usize) {
        for _ in 0..times {
            self.overhand_shuffle();
        }
    }

    /// Deals a card from the top of the deck.
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    /// Deals n amount of cards from the top of the deck
    /// If n > deck.len() return None
    /// if n == 0 return Some(Vec::new())
    /// if n <= deck.len() return Some(Vec<Card>)
    pub fn deal_n(&mut self, n: usize) -> Option<Vec<Card>> {
        let mut cards = Vec::new();
        for _ in 0..n {
            let card = self.deal()?;
            cards.push(card);
        }
        Some(cards)
    }

    /// Deals a card from the bottom of the deck.
    pub fn deal_bottom(&mut self) -> Option<Card> {
        self.cards.pop_back()
    }

    /// Deals n amount of cards from the bottom of the deck
    pub fn deal_n_bottom(&mut self, n: usize) -> Option<Vec<Card>> {
        let mut cards = Vec::new();
        for _ in 0..n {
            let card = self.deal_bottom()?;
            cards.push(card);
        }
        Some(cards)
    }

    /// Adds a card to the top of the deck.
    pub fn add_card(&mut self, card: Card) {
        self.cards.push_front(card);
    }

    /// Adds a vec of cards to the top of the deck.
    pub fn add_cards(&mut self, cards: Vec<Card>) {
        for card in cards.into_iter().rev() {
            self.add_card(card);
        }
    }

    /// Adds a card to the bottom of the deck.
    pub fn add_card_bottom(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    /// Adds a vec of cards to the bottom of the deck.
    pub fn add_cards_bottom(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.add_card_bottom(card);
        }
    }

    /// Peeks at the top card of the deck without removing it.
    pub fn peek(&self) -> Option<&Card> {
        self.cards.front()
    }

    /// Peeks at the bottom card of the deck without removing it.
    pub fn peek_bottom(&self) -> Option<&Card> {
        self.cards.back()
    }

    /// Peeks at index of the deck without removing it.
    pub fn peek_at(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }

    /// Clears the deck of all cards.
    pub fn clear(&mut self) {
        self.cards.clear();
    }

    /// Returns a string representation of the deck with the specified delimiter.
    pub fn as_str_delimiter(&self, delimiter: char) -> String {
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

    /// Returns a CSV representation of the deck.
    pub fn as_csv(&self) -> String {
        let mut csv = "Rank,Suit\n".to_string();
        for card in &self.cards {
            csv.push_str(&card.as_csv_row());
            csv.push('\n');
        }
        csv
    }

    /// Creates a Deck from a CSV string.
    pub fn from_csv(s: &str) -> Result<Self, String> {
        let mut cards = VecDeque::new();
        for (i, line) in s.lines().enumerate() {
            if i == 0 {
                continue;
            }
            if line.trim().is_empty() {
                continue;
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

impl Index<usize> for Deck {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

impl IndexMut<usize> for Deck {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cards[index]
    }
}

impl FromIterator<Card> for Deck {
    /// Creates a Deck from an iterator of Cards.
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        let cards: VecDeque<Card> = iter.into_iter().collect();
        Deck::new(cards)
    }
}
