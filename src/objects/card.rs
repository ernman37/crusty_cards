use serde::{Deserialize, Serialize};
use std::convert::{From, TryFrom};
use std::fmt;
use std::str::FromStr;

use super::color::Color;
use super::rank::Rank;
use super::suit::Suit;

/// Represents a playing card with a suit and rank.
///
/// Cards are immutable value types that can be compared, hashed, and serialized.
/// They implement `Copy` for efficient passing by value.
///
/// # Examples
///
/// ```rust
/// use crusty_cards::{Card, Suit, Rank};
///
/// // Create a card
/// let card = Card::new(Suit::Spades, Rank::Ace);
///
/// // Parse from string (supports multiple formats)
/// let card: Card = "A♠".parse().unwrap();
/// let card: Card = "♠A".parse().unwrap();
/// let card: Card = "10♥".parse().unwrap();
/// println!("{}", card);  // "T♥"
///
/// // Access properties
/// assert_eq!(card.suit(), Suit::Hearts);
/// assert_eq!(card.rank(), Rank::Ten);
/// ```
///
/// # Numeric Conversion
///
/// Cards can be converted to/from integers (0-55) for compact storage:
///
/// ```rust
/// use crusty_cards::{Card, Suit, Rank};
///
/// let card = Card::new(Suit::Hearts, Rank::Two);
/// let value: u8 = card.into();
/// let restored = Card::try_from(value).unwrap();
/// assert_eq!(card, restored);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl fmt::Display for Card {
    /// Formats the card as a string.
    ///
    /// Formats as "<rank><suit>" (e.g., "A♠", "10♥").
    /// rank: The rank of the card represented as single ascii char (e.g., "A", "T").
    /// suit: The suit of the card represented as single unicode char (e.g., "♠", "♥").
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Ace);
    /// assert_eq!(card.to_string(), "A♥");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    /// Creates a new card with the given suit and rank.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    ///
    /// let ace_of_spades = Card::new(Suit::Spades, Rank::Ace);
    /// ```
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    /// Displays the card using ASCII art.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    ///
    /// let card = Card::new(Suit::Hearts, Rank::King);
    /// println!("{}", card.display_ascii());
    /// // ┌─────┐
    /// // │K    │
    /// // │  ♥  │
    /// // │    K│
    /// // └─────┘
    /// ```
    pub fn display_ascii(&self) -> String {
        format!(
            "┌─────┐\n│{}   │\n│  {}  │\n│   {}│\n└─────┘",
            self.rank, self.suit, self.rank
        )
    }

    /// Returns the color of the card (Red or Black).
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank, Color};
    ///
    /// let red_card = Card::new(Suit::Hearts, Rank::Ace);
    /// let black_card = Card::new(Suit::Spades, Rank::King);
    /// assert_eq!(red_card.color(), Color::Red);
    /// assert_eq!(black_card.color(), Color::Black);
    /// ```
    /// Hearts and Diamonds are Red; Clubs and Spades are Black.
    pub fn color(&self) -> Color {
        self.suit.color()
    }

    /// Returns the `Suit` of the card. (Hearts, Diamonds, Clubs, Spades)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    ///
    /// let card = Card::new(Suit::Hearts, Rank::Ace);
    /// assert_eq!(card.suit(), Suit::Hearts);
    /// ```
    pub fn suit(&self) -> Suit {
        self.suit
    }

    /// Returns the `Rank` of the card. (Ace, Two, ..., King, Joker)
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    ///
    /// let card = Card::new(Suit::Hearts, Rank::Ace);
    /// assert_eq!(card.rank(), Rank::Ace);
    /// ```
    pub fn rank(&self) -> Rank {
        self.rank
    }

    /// Returns `true` if the card is an `Ace`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    /// let ace_card = Card::new(Suit::Hearts, Rank::Ace);
    /// let non_ace_card = Card::new(Suit::Spades, Rank::Ten);
    /// assert_eq!(ace_card.is_ace(), true);
    /// assert_eq!(non_ace_card.is_ace(), false);
    /// ```
    pub fn is_ace(&self) -> bool {
        matches!(self.rank, Rank::Ace)
    }

    /// Returns `true` if the card is a face card (Jack, Queen, or King).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    /// let jack_card = Card::new(Suit::Hearts, Rank::Jack);
    /// let non_face_card = Card::new(Suit::Spades, Rank::Ten);
    /// assert_eq!(jack_card.is_face_card(), true);
    /// assert_eq!(non_face_card.is_face_card(), false);
    /// ```
    pub fn is_face_card(&self) -> bool {
        matches!(self.rank, Rank::Jack | Rank::Queen | Rank::King)
    }

    /// Returns `true` if the card is a number card (2-10).
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    /// let two_card = Card::new(Suit::Hearts, Rank::Two);
    /// let non_value_card = Card::new(Suit::Spades, Rank::Jack);
    /// assert_eq!(two_card.is_value_card(), true);
    /// assert_eq!(non_value_card.is_value_card(), false);
    /// ```
    pub fn is_value_card(&self) -> bool {
        matches!(
            self.rank,
            Rank::Two
                | Rank::Three
                | Rank::Four
                | Rank::Five
                | Rank::Six
                | Rank::Seven
                | Rank::Eight
                | Rank::Nine
                | Rank::Ten
        )
    }

    /// Returns `true` if the card is a Joker.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let joker_card = Card::new(Suit::Hearts, Rank::Joker);
    /// let non_joker_card = Card::new(Suit::Hearts, Rank::Ace);
    /// assert_eq!(joker_card.is_joker(), true);
    /// assert_eq!(non_joker_card.is_joker(), false);
    /// ```
    pub fn is_joker(&self) -> bool {
        matches!(self.rank, Rank::Joker)
    }

    /// Returns `true` if this card has the same rank as another card.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card1 = Card::new(Suit::Hearts, Rank::Ace);
    /// let card2 = Card::new(Suit::Diamonds, Rank::Ace);
    /// let card3 = Card::new(Suit::Clubs, Rank::King);
    /// assert_eq!(card1.is_same_rank(&card2), true);
    /// assert_eq!(card1.is_same_rank(&card3), false);
    /// ```
    pub fn is_same_rank(&self, other: &Card) -> bool {
        self.rank == other.rank
    }

    /// Returns `true` if this card has the same suit as another card.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card1 = Card::new(Suit::Hearts, Rank::Ace);
    /// let card2 = Card::new(Suit::Hearts, Rank::King);
    /// let card3 = Card::new(Suit::Clubs, Rank::Ace);
    /// assert_eq!(card1.is_same_suit(&card2), true);
    /// assert_eq!(card1.is_same_suit(&card3), false);
    /// ```
    pub fn is_same_suit(&self, other: &Card) -> bool {
        self.suit == other.suit
    }

    /// Returns `true` if this card has the same color as another card.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    /// let red_card1 = Card::new(Suit::Hearts, Rank::Ace);
    /// let red_card2 = Card::new(Suit::Diamonds, Rank::King);
    /// let black_card = Card::new(Suit::Spades, Rank::Queen);
    /// assert_eq!(red_card1.is_same_color(&red_card2), true);
    /// assert_eq!(red_card1.is_same_color(&black_card), false);
    /// ```
    pub fn is_same_color(&self, other: &Card) -> bool {
        self.color() == other.color()
    }

    /// Returns a CSV representation of the card as "Rank,Suit".
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    ///
    /// let card = Card::new(Suit::Hearts, Rank::Queen);
    /// assert_eq!(card.as_csv_row(), "Q,♥");
    /// ```
    pub fn as_csv_row(&self) -> String {
        format!("{},{}", self.rank, self.suit)
    }

    /// Creates a Card from a CSV row in "Rank,Suit" format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    ///
    /// let card = Card::from_csv_row("Q,♥").unwrap();
    /// assert_eq!(card, Card::new(Suit::Hearts, Rank::Queen));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the row format is invalid or contains unknown rank/suit values.
    pub fn from_csv_row(row: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = row.split(',').collect();
        if parts.len() != 2 {
            return Err("Invalid CSV row format");
        }
        let rank = Rank::from_str(parts[0]).map_err(|_| "Invalid rank")?;
        let suit = Suit::from_str(parts[1]).map_err(|_| "Invalid suit")?;
        Ok(Card::new(suit, rank))
    }
}

// ============================================================================
// Numeric Conversions
// ============================================================================
//
// Cards are encoded as: suit_value * 14 + rank_value
// This gives a unique value 0-55 for each possible card (including jokers).

impl From<Card> for u8 {
    /// Converts a Card to a u8 index (0-55).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: u8 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn from(card: Card) -> Self {
        let suit_value = card.suit.value();
        let rank_value = card.rank.value();
        suit_value * 14 + rank_value
    }
}

impl From<Card> for i8 {
    /// Converts a Card to an i8 index (0-55).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: i8 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn from(card: Card) -> Self {
        u8::from(card) as i8
    }
}

impl From<Card> for u16 {
    /// Converts a Card to a u16 index (0-55).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: u16 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for i16 {
    /// Converts a Card to an i16 index (0-55).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: i16 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for u32 {
    /// Converts a Card to a u32 index (0-55).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: u32 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for i32 {
    /// Converts a Card to an i32 index (0-55).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: i32 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for u64 {
    /// Converts a Card to a u64 index (0-55).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: u64 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for i64 {
    /// Converts a Card to an i64 index (0-55).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: i64 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for usize {
    /// Converts a Card to a usize index (0-55).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: usize = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for isize {
    /// Converts a Card to an isize index (0-55).
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: isize = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl TryFrom<u8> for Card {
    type Error = &'static str;

    /// Converts a u8 value to a Card.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: u8 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..56).contains(&value) {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }

        let suit = Suit::ALL[(value / 14) as usize];
        let rank = Rank::ALL[(value % 14) as usize];

        Ok(Card::new(suit, rank))
    }
}

impl TryFrom<i8> for Card {
    type Error = &'static str;

    /// Converts an i8 value to a Card.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: i8 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if !(0..56).contains(&value) {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<u16> for Card {
    type Error = &'static str;

    /// Converts a u16 value to a Card.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: u16 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if !(0..56).contains(&value) {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<i16> for Card {
    type Error = &'static str;

    /// Converts an i16 value to a Card.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: i16 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if !(0..56).contains(&value) {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<u32> for Card {
    type Error = &'static str;

    /// Converts a u32 value to a Card.
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: u32 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if !(0..56).contains(&value) {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<i32> for Card {
    type Error = &'static str;

    /// Converts an i32 value to a Card.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: i32 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if !(0..56).contains(&value) {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<u64> for Card {
    type Error = &'static str;

    /// Converts a u64 value to a Card.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: u64 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if !(0..56).contains(&value) {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<i64> for Card {
    type Error = &'static str;

    /// Converts an i64 value to a Card.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: i64 = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if !(0..56).contains(&value) {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<usize> for Card {
    type Error = &'static str;

    /// Converts a usize value to a Card.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: usize = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if !(0..56).contains(&value) {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<isize> for Card {
    type Error = &'static str;

    /// Converts an isize value to a Card.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let index: isize = card.into();
    /// assert_eq!(index, 0);
    /// ```
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if !(0..56).contains(&value) {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl FromStr for Card {
    type Err = String;

    /// Converts a string to a Card.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Card, Suit, Rank};
    /// let card = Card::new(Suit::Hearts, Rank::Two);
    /// let s = card.to_string();
    /// let parsed: Card = s.parse().unwrap();
    /// assert_eq!(card, parsed);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Input string is empty".to_string());
        }
        if s.chars().count() < 2 {
            return Err("Input string is too short to represent a card".to_string());
        }

        let char_indices: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
        for &split_pos in &char_indices[1..] {
            let (left, right) = s.split_at(split_pos);
            if let (Ok(suit), Ok(rank)) = (Suit::from_str(left), Rank::from_str(right)) {
                return Ok(Card::new(suit, rank));
            }
            if let (Ok(rank), Ok(suit)) = (Rank::from_str(left), Suit::from_str(right)) {
                return Ok(Card::new(suit, rank));
            }
        }
        Err(format!("Invalid card string: {}", s))
    }
}
