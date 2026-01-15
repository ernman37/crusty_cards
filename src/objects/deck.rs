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

/// A collection of playing cards with deck manipulation operations.
///
/// `Deck` is backed by a `VecDeque<Card>`, providing efficient operations
/// at both ends (dealing from top/bottom, adding cards).
///
/// # Creating Decks
///
/// ```rust
/// use crusty_cards::{Deck, Standard52, Card, Suit, Rank};
///
/// // From a factory (most common)
/// let deck = Deck::from_factory(Standard52);
///
/// // From a string
/// let deck: Deck = "A♠ K♠ Q♠ J♠".parse().unwrap();
///
/// // Empty deck
/// let deck = Deck::default();
/// ```
///
/// # Shuffling
///
/// ```rust
/// use crusty_cards::{Deck, Standard52};
///
/// let mut deck = Deck::from_factory(Standard52);
/// deck.shuffle();           // Random shuffle
/// deck.riffle_shuffle();    // Riffle (interleave) shuffle
/// deck.overhand_shuffle();  // Overhand shuffle
/// ```
///
/// # Dealing
///
/// ```rust
/// use crusty_cards::{Deck, Standard52};
///
/// let mut deck = Deck::from_factory(Standard52);
/// let card = deck.deal();           // From top
/// let card = deck.deal_bottom();    // From bottom
/// let cards = deck.deal_n(5);       // Multiple cards
/// ```
///
/// # Operator Overloads
///
/// ```rust
/// use crusty_cards::{Deck, Card, Suit, Rank};
///
/// let mut deck = Deck::default();
/// let card = Card::new(Suit::Spades, Rank::Ace);
///
/// deck += card;                     // Add card
/// deck -= card;                     // Remove card
/// let doubled = deck.clone() * 2;   // Duplicate
/// let combined = deck.clone() + Deck::default();  // Combine decks
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl fmt::Display for Deck {
    /// Formats the deck as a string.
    /// Utilizes the `Card`'s `Display` implementation and the Deck `as_str_delimiter(' ')` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    ///
    /// let mut deck = Deck::default();
    /// let card = Card::new(Suit::Spades, Rank::Ace);
    ///
    /// deck += card;                     // Add card
    /// deck -= card;                     // Remove card
    /// let doubled = deck.clone() * 2;   // Duplicate
    /// let combined = deck.clone() + Deck::default();  // Combine decks
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str_delimiter(' '))
    }
}

impl Deck {
    /// Creates a new deck with the given cards.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// use std::collections::VecDeque;
    ///
    /// let cards: VecDeque<Card> = vec![
    ///     Card::new(Suit::Spades, Rank::Ace),
    ///     Card::new(Suit::Hearts, Rank::King),
    /// ].into();
    ///
    /// let deck = Deck::new(cards);
    /// assert_eq!(deck.len(), 2);
    /// ```
    pub fn new(cards: VecDeque<Card>) -> Self {
        Deck { cards }
    }

    /// Creates a new deck using a [`DeckFactory`].
    ///
    /// This is the recommended way to create standard decks.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::{Deck, Standard52, Standard54};
    ///
    /// let deck = Deck::from_factory(Standard52);  // 52 cards
    /// let deck = Deck::from_factory(Standard54);  // 54 cards with jokers
    /// ```
    pub fn from_factory<F>(factory: F) -> Self
    where
        F: DeckFactory,
    {
        Self {
            cards: factory.generate(),
        }
    }

    /// Returns an iterator over references to the cards.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::{Deck, Standard52};
    ///
    /// let deck = Deck::from_factory(Standard52);
    /// for card in deck.iter() {
    ///     println!("{}", card);
    /// }
    /// ```
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, Card> {
        self.cards.iter()
    }

    /// Returns an iterator over mutable references to the cards.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Standard52, Card, Suit, Rank};
    ///
    /// let mut deck = Deck::from_factory(Standard52);
    /// for card in deck.iter_mut() {
    ///     *card = Card::new(Suit::Diamonds, Rank::Ace);
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> std::collections::vec_deque::IterMut<'_, Card> {
        self.cards.iter_mut()
    }

    /// Cuts the deck at the given index, moving cards from index to end to the top.
    ///
    /// Returns `false` if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::Deck;
    ///
    /// let mut deck: Deck = "A♠ K♠ Q♠ J♠".parse().unwrap();
    /// deck.cut(2);  // Q♠ and J♠ move to top
    /// assert_eq!(deck.to_string(), "Q♠ J♠ A♠ K♠");
    /// ```
    pub fn cut(&mut self, index: usize) -> bool {
        if index >= self.len() {
            return false;
        }
        let mut top = self.cards.split_off(index);
        top.append(&mut self.cards);
        self.cards = top;
        true
    }

    /// Returns `true` if the deck contains the specified card.
    ///
    /// # Examples
    ///
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    ///
    /// let mut deck = Deck::default();
    /// let card = Card::new(Suit::Spades, Rank::Ace);
    ///
    /// deck += card;
    /// assert_eq!(deck.contains(&card), true);
    /// ```
    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    /// Returns the index of the first occurrence of the card, or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    ///
    /// let mut deck = Deck::default();
    /// let card = Card::new(Suit::Spades, Rank::Ace);
    ///
    /// deck += card;
    /// assert_eq!(deck.find(&card), Some(0));
    /// ```
    pub fn find(&self, card: &Card) -> Option<usize> {
        self.cards.iter().position(|c| c == card)
    }

    /// Inserts a card at the specified index.
    ///
    /// Returns `false` if index is out of bounds (> len).
    ///
    /// # Examples
    ///
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    ///
    /// let mut deck = Deck::default();
    /// let card = Card::new(Suit::Spades, Rank::Ace);
    ///
    /// deck += card;
    /// assert_eq!(deck.insert_at(card, 0), true);
    /// assert_eq!(deck.insert_at(card, 5), false);
    /// ```
    pub fn insert_at(&mut self, card: Card, index: usize) -> bool {
        if index > self.cards.len() {
            return false;
        }
        self.cards.insert(index, card);
        true
    }

    /// Removes and returns the card at the specified index.
    ///
    /// Returns `None` if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    ///
    /// let mut deck = Deck::default();
    /// let card = Card::new(Suit::Spades, Rank::Ace);
    ///
    /// deck += card;
    /// assert_eq!(deck.remove_at(0), Some(card));
    /// assert_eq!(deck.remove_at(0), None);
    /// ```
    pub fn remove_at(&mut self, index: usize) -> Option<Card> {
        if index >= self.cards.len() {
            return None;
        }
        self.cards.remove(index)
    }

    /// Reverses the order of cards in the deck.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::Deck;
    ///
    /// let mut deck: Deck = "A♠ K♠ Q♠ J♠".parse().unwrap();
    /// deck.reverse();
    /// assert_eq!(deck.to_string(), "J♠ Q♠ K♠ A♠");
    /// ```
    pub fn reverse(&mut self) {
        self.cards = self.cards.iter().cloned().rev().collect();
    }

    /// Counts occurrences of a specific card in the deck.
    ///
    /// Useful for decks with duplicate cards (e.g., Pinochle).
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    ///
    /// let mut deck = Deck::default();
    /// let card = Card::new(Suit::Spades, Rank::Ace);
    ///
    /// deck += card;
    /// assert_eq!(deck.count(&card), 1);
    /// ```
    pub fn count(&self, card: &Card) -> usize {
        self.cards.iter().filter(|&&c| c == *card).count()
    }

    /// Returns the number of cards in the deck.
    ///
    /// # Examples
    ///
    /// ```
    /// use crusty_cards::Deck;
    ///
    /// let mut deck = Deck::default();
    /// assert_eq!(deck.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Splits the deck into two decks at the given index.
    ///
    /// - First deck: cards `[0, index)`
    /// - Second deck: cards `[index, len)`
    ///
    /// If index > len, returns (clone of self, empty deck).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::Deck;
    ///
    /// let deck: Deck = "A♠ K♠ Q♠ J♠".parse().unwrap();
    /// let (left, right) = deck.split_at(2);
    /// assert_eq!(left.to_string(), "A♠ K♠");
    /// assert_eq!(right.to_string(), "Q♠ J♠");
    /// ```
    pub fn split_at(&self, index: usize) -> (Self, Self) {
        let index = index.min(self.cards.len());
        let left: VecDeque<Card> = self.cards.iter().take(index).cloned().collect();
        let right: VecDeque<Card> = self.cards.iter().skip(index).cloned().collect();
        (Deck::new(left), Deck::new(right))
    }

    /// Returns `true` if the deck contains no cards.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::Deck;
    /// let mut deck = Deck::default();
    /// assert_eq!(deck.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Randomly shuffles the deck using Fisher-Yates algorithm.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Deck;
    /// let mut deck = Deck::default();
    /// deck.shuffle();
    /// ```
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        let mut cards_vec: Vec<Card> = self.cards.drain(..).collect();
        cards_vec.shuffle(&mut rng);
        self.cards = VecDeque::from(cards_vec);
    }

    /// Shuffles the deck multiple times.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Deck;
    /// let mut deck = Deck::default();
    /// deck.shuffle_times(3);
    /// ```
    pub fn shuffle_times(&mut self, times: usize) {
        for _ in 0..times {
            self.shuffle();
        }
    }

    /// Performs a riffle (interleave) shuffle.
    ///
    /// Splits the deck in half and interleaves the cards.
    /// This is a deterministic shuffle (not random).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Deck;
    /// let mut deck = Deck::default();
    /// deck.riffle_shuffle();
    /// ```
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

    /// Performs a riffle shuffle multiple times.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Deck;
    /// let mut deck = Deck::default();
    /// deck.riffle_shuffle_times(3);
    /// ```
    pub fn riffle_shuffle_times(&mut self, times: usize) {
        for _ in 0..times {
            self.riffle_shuffle();
        }
    }

    /// Performs an overhand shuffle.
    ///
    /// Simulates the common overhand shuffling technique by repeatedly
    /// moving small random groups of cards from top to a new pile.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Deck;
    /// let mut deck = Deck::default();
    /// deck.overhand_shuffle();
    /// ```
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

    /// Performs an overhand shuffle multiple times.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Deck;
    /// let mut deck = Deck::default();
    /// deck.overhand_shuffle_times(3);
    /// ```
    pub fn overhand_shuffle_times(&mut self, times: usize) {
        for _ in 0..times {
            self.overhand_shuffle();
        }
    }

    /// Deals (removes and returns) the top card.
    ///
    /// Returns `None` if the deck is empty.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let card = deck.deal();
    /// assert_eq!(card, Some(Card::new(Suit::Hearts, Rank::Ace)));
    /// ```
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    /// Deals (removes and returns) n cards from the top.
    ///
    /// Returns `None` if fewer than n cards remain.
    /// Returns `Some(Vec::new())` if n is 0.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// deck.add_card(Card::new(Suit::Hearts, Rank::King));
    /// let cards = deck.deal_n(2);
    /// assert_eq!(cards, Some(vec![
    ///     Card::new(Suit::Hearts, Rank::King),
    ///     Card::new(Suit::Hearts, Rank::Ace)
    /// ]));
    /// ```
    pub fn deal_n(&mut self, n: usize) -> Option<Vec<Card>> {
        let mut cards = Vec::new();
        for _ in 0..n {
            let card = self.deal()?;
            cards.push(card);
        }
        Some(cards)
    }

    /// Deals (removes and returns) the bottom card.
    ///
    /// Returns `None` if the deck is empty.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let card = deck.deal_bottom();
    /// assert_eq!(card, Some(Card::new(Suit::Hearts, Rank::Ace)));
    /// ```
    pub fn deal_bottom(&mut self) -> Option<Card> {
        self.cards.pop_back()
    }

    /// Deals n amount of cards from the bottom of the deck
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let cards = deck.deal_n_bottom(1);
    /// assert_eq!(cards, Some(vec![Card::new(Suit::Hearts, Rank::Ace)]));
    /// ```
    pub fn deal_n_bottom(&mut self, n: usize) -> Option<Vec<Card>> {
        let mut cards = Vec::new();
        for _ in 0..n {
            let card = self.deal_bottom()?;
            cards.push(card);
        }
        Some(cards)
    }

    /// Deals a card from the specified index
    ///
    /// Returns `None` if the index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let card = deck.deal_from(0);
    /// assert_eq!(card, Some(Card::new(Suit::Hearts, Rank::Ace)));
    /// ```
    pub fn deal_from(&mut self, index: usize) -> Option<Card> {
        self.cards.remove(index)
    }

    /// Deals n cards from the specified index
    ///
    /// Returns `None` if the index is out of bounds or if there are not enough
    ///   cards remaining
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let cards = deck.deal_n_from(0, 1);
    /// assert_eq!(cards, Some(vec![Card::new(Suit::Hearts, Rank::Ace)]));
    /// ```
    pub fn deal_n_from(&mut self, index: usize, n: usize) -> Option<Vec<Card>> {
        let mut cards = Vec::new();
        for _ in 0..n {
            let card = self.deal_from(index)?;
            cards.push(card);
        }
        Some(cards)
    }

    /// Adds a card to the top of the deck.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// let card: Card = Card::new(Suit::Hearts, Rank::Ace);
    /// deck.add_card(card);
    /// assert_eq!(deck.peek(), Some(&Card::new(Suit::Hearts, Rank::Ace)));
    /// ```
    pub fn add_card(&mut self, card: Card) {
        self.cards.push_front(card);
    }

    /// Adds a vec of cards to the top of the deck.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card};
    /// let mut deck = Deck::default();
    /// let card1: Card = "KingHearts".parse().unwrap();
    /// let card2: Card = "AceSpades".parse().unwrap();
    /// deck.add_cards(vec![card1, card2]);
    /// assert_eq!(deck.len(), 2);
    /// ```
    pub fn add_cards(&mut self, cards: Vec<Card>) {
        for card in cards.into_iter().rev() {
            self.add_card(card);
        }
    }

    /// Adds a card to the bottom of the deck.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// let card: Card = Card::new(Suit::Hearts, Rank::Ace);
    /// deck.add_card_bottom(card);
    /// assert_eq!(deck.peek_bottom(), Some(&Card::new(Suit::Hearts, Rank::Ace)));
    /// ```
    pub fn add_card_bottom(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    /// Adds a vec of cards to the bottom of the deck.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// let card1: Card = Card::new(Suit::Hearts, Rank::Ace);
    /// let card2: Card = Card::new(Suit::Hearts, Rank::King);
    /// deck.add_cards_bottom(vec![card1, card2]);
    /// assert_eq!(deck.len(), 2);
    /// ```
    pub fn add_cards_bottom(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.add_card_bottom(card);
        }
    }

    /// Peeks at the top card of the deck without removing it.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// assert_eq!(deck.peek(), Some(&Card::new(Suit::Hearts, Rank::Ace)));
    /// ```
    pub fn peek(&self) -> Option<&Card> {
        self.cards.front()
    }

    /// Peeks at the bottom card of the deck without removing it.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card_bottom(Card::new(Suit::Hearts, Rank::Ace));
    /// assert_eq!(deck.peek_bottom(), Some(&Card::new(Suit::Hearts, Rank::Ace)));
    /// ```
    pub fn peek_bottom(&self) -> Option<&Card> {
        self.cards.back()
    }

    /// Peeks at index of the deck without removing it.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// assert_eq!(deck.peek_at(0), Some(&Card::new(Suit::Hearts, Rank::Ace)));
    /// ```
    pub fn peek_at(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }

    /// Clears the deck of all cards.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// assert_eq!(deck.len(), 1);
    /// deck.clear();
    /// assert_eq!(deck.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.cards.clear();
    }

    /// Returns a string representation of the deck with the specified delimiter.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// deck.add_card(Card::new(Suit::Hearts, Rank::King));
    /// assert_eq!(deck.as_str_delimiter(','), "K♥,A♥");
    /// ```
    pub fn as_str_delimiter(&self, delimiter: char) -> String {
        self.cards
            .iter()
            .map(|card| format!("{}", card))
            .collect::<Vec<String>>()
            .join(&delimiter.to_string())
    }

    /// Creates a Deck from a string representation of cards separated by a delimiter.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let deck = Deck::from_str_delimiter("A♥,K♥", ',').unwrap();
    /// assert_eq!(deck.len(), 2);
    /// ```
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
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let json = deck.to_json().unwrap();
    /// ```
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serializes the deck to a pretty-printed JSON string.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let json = deck.to_json_pretty().unwrap();
    /// ```
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Creates a Deck from a JSON string.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let json = deck.to_json().unwrap();
    /// let deck2 = Deck::from_json(&json).unwrap();
    /// assert_eq!(deck, deck2);
    /// ```
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    /// Serializes the deck to a YAML string.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let yaml = deck.to_yaml().unwrap();
    /// ```
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    /// Creates a Deck from a YAML string.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let yaml = deck.to_yaml().unwrap();
    /// let deck2 = Deck::from_yaml(&yaml).unwrap();
    /// assert_eq!(deck, deck2);
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let csv = deck.to_csv();
    /// ```
    pub fn to_csv(&self) -> String {
        let mut csv = "Rank,Suit\n".to_string();
        for card in &self.cards {
            csv.push_str(&card.as_csv_row());
            csv.push('\n');
        }
        csv
    }

    /// Creates a Deck from a CSV string.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Rank, Suit};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let csv = deck.to_csv();
    /// let deck2 = Deck::from_csv(&csv).unwrap();
    /// assert_eq!(deck, deck2);
    /// ```
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
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// let new_deck = deck + Card::new(Suit::Hearts, Rank::Ace);
    /// assert_eq!(new_deck.len(), 1);
    /// ```
    fn add(self, rhs: Card) -> Deck {
        let mut new_deck = self.clone();
        new_deck.add_card(rhs);
        new_deck
    }
}

impl AddAssign<Card> for Deck {
    /// Adds specified Card to the deck (top) in place.
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// deck += Card::new(Suit::Hearts, Rank::Ace);
    /// assert_eq!(deck.len(), 1);
    /// ```
    fn add_assign(&mut self, rhs: Card) {
        self.add_card(rhs);
    }
}

impl Sub<Card> for Deck {
    type Output = Deck;

    /// Removes specified Card from the deck, returns a new Deck.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let new_deck = deck - Card::new(Suit::Hearts, Rank::Ace);
    /// assert_eq!(new_deck.len(), 0);
    /// ```
    fn sub(self, rhs: Card) -> Deck {
        let mut new_deck = self.clone();
        new_deck.cards.retain(|c| c != &rhs);
        new_deck
    }
}

impl SubAssign<Card> for Deck {
    /// Removes specified Card from the deck in place.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// deck -= Card::new(Suit::Hearts, Rank::Ace);
    /// assert_eq!(deck.len(), 0);
    /// ```
    fn sub_assign(&mut self, rhs: Card) {
        *self = self.clone() - rhs;
    }
}

impl Mul<usize> for Deck {
    type Output = Deck;

    /// Multiplies the deck by duplicating its cards n times, returns a new Deck.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let new_deck = deck * 2;
    /// assert_eq!(new_deck.len(), 2);
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// deck *= 2;
    /// assert_eq!(deck.len(), 2);
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck1 = Deck::default();
    /// let mut deck2 = Deck::default();
    /// deck1.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// deck2.add_card(Card::new(Suit::Diamonds, Rank::King));
    /// let new_deck = deck1 + deck2;
    /// assert_eq!(new_deck.len(), 2);
    /// ```
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
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// let mut deck2 = Deck::default();
    /// deck2.add_card(Card::new(Suit::Diamonds, Rank::King));
    /// deck += deck2;
    /// assert_eq!(deck.len(), 2);
    /// ```
    fn add_assign(&mut self, rhs: Deck) {
        for card in rhs.cards {
            self.add_card(card);
        }
    }
}

impl Sub<Deck> for Deck {
    type Output = Deck;

    /// Removes cards from rhs Deck from self Deck, returns a new Deck.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck1 = Deck::default();
    /// let mut deck2 = Deck::default();
    /// deck1.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// deck1.add_card(Card::new(Suit::Diamonds, Rank::King));
    /// deck2.add_card(Card::new(Suit::Diamonds, Rank::King));
    /// let new_deck = deck1 - deck2;
    /// assert_eq!(new_deck.len(), 1);
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck1 = Deck::default();
    /// let mut deck2 = Deck::default();
    /// deck1.add_card(Card::new(Suit::Hearts, Rank::Ace));
    /// deck1.add_card(Card::new(Suit::Diamonds, Rank::King));
    /// deck2.add_card(Card::new(Suit::Diamonds, Rank::King));
    /// deck1 -= deck2;
    /// assert_eq!(deck1.len(), 1);
    /// ```
    fn sub_assign(&mut self, rhs: Deck) {
        *self = self.clone() - rhs;
    }
}

impl TryFrom<Vec<usize>> for Deck {
    type Error = String;

    /// Tries to create a Deck from a vector of usize indices.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let values = vec![0, 1, 2];
    /// let deck = Deck::try_from(values).unwrap();
    /// assert_eq!(deck.len(), 3);
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Two));
    /// let indices: Vec<usize> = deck.into();
    /// assert_eq!(indices, vec![0]);
    /// ```
    fn from(deck: Deck) -> Self {
        deck.cards.iter().map(|card| usize::from(*card)).collect()
    }
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = std::collections::vec_deque::IntoIter<Card>;

    /// Consumes the Deck and returns an iterator over its cards.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// use std::collections::VecDeque;
    /// let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Two)]);
    /// let deck = Deck::new(cards);
    /// let mut iter = deck.into_iter();
    /// assert_eq!(iter.next(), Some(Card::new(Suit::Hearts, Rank::Two)));
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'a> IntoIterator for &'a Deck {
    type Item = &'a Card;
    type IntoIter = std::collections::vec_deque::Iter<'a, Card>;

    /// Returns an iterator over references to the cards in the deck.
    ///
    /// This allows using `for card in &deck` syntax.
    ///
    /// # Examples
    ///
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    ///
    /// let deck: Deck = "A♠ K♠".parse().unwrap();
    ///
    /// // Using for-in loop (most common usage)
    /// for card in &deck {
    ///     println!("{}", card);
    /// }
    ///
    /// // Deck is still usable after iteration
    /// assert_eq!(deck.len(), 2);
    ///
    /// // Using iterator directly
    /// let mut iter = (&deck).into_iter();
    /// assert_eq!(iter.next(), Some(&Card::new(Suit::Spades, Rank::Ace)));
    /// assert_eq!(iter.next(), Some(&Card::new(Suit::Spades, Rank::King)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Deck {
    type Item = &'a mut Card;
    type IntoIter = std::collections::vec_deque::IterMut<'a, Card>;

    /// Returns an iterator over the mutable cards in the referenced Deck.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// use std::collections::VecDeque;
    /// let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Two)]);
    /// let mut deck = Deck::new(cards);
    /// for card in deck.iter_mut() {
    ///     *card = Card::new(Suit::Diamonds, Rank::Ace);
    /// }
    /// assert_eq!(deck[0], Card::new(Suit::Diamonds, Rank::Ace));
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl FromStr for Deck {
    type Err = String;

    /// Creates a Deck from a string representation of cards separated by spaces.
    ///
    /// Example:
    /// ```
    /// use crusty_cards::Deck;
    /// use std::str::FromStr;
    /// let deck = Deck::from_str("2H 3D 4C").unwrap();
    /// assert_eq!(deck.len(), 3);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Deck::from_str_delimiter(s, ' ')
    }
}

impl Index<usize> for Deck {
    type Output = Card;

    /// Returns a reference to the card at the given index.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Two));
    /// let card = deck[0];
    /// assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

impl IndexMut<usize> for Deck {
    /// Returns a mutable reference to the card at the given index.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let mut deck = Deck::default();
    /// deck.add_card(Card::new(Suit::Hearts, Rank::Two));
    /// deck[0] = Card::new(Suit::Diamonds, Rank::Ace);
    /// assert_eq!(deck[0], Card::new(Suit::Diamonds, Rank::Ace));
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cards[index]
    }
}

impl FromIterator<Card> for Deck {
    /// Creates a Deck from an iterator of Cards.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Deck, Card, Suit, Rank};
    /// let cards = vec![Card::new(Suit::Hearts, Rank::Two)];
    /// let deck = Deck::from_iter(cards);
    /// assert_eq!(deck.len(), 1);
    /// ```
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        let cards: VecDeque<Card> = iter.into_iter().collect();
        Deck::new(cards)
    }
}
