use serde::{Deserialize, Serialize};
use std::convert::{From, TryFrom};
use std::fmt;
use std::str::FromStr;

use super::color::Color;
use super::rank::Rank;
use super::suit::Suit;

/// Represents a playing card with a suit and rank.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    /// Creates a new card with the given suit and rank.
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    /// Displays the card using ASCII art.
    pub fn display_ascii(&self) -> String {
        format!(
            "┌─────┐\n│{}   │\n│  {}  │\n│   {}│\n└─────┘",
            self.rank, self.suit, self.rank
        )
    }

    /// Returns the color of the card.
    pub fn color(&self) -> Color {
        self.suit.color()
    }

    /// Returns the suit of the card.
    pub fn suit(&self) -> Suit {
        self.suit
    }

    /// Returns the rank of the card.
    pub fn rank(&self) -> Rank {
        self.rank
    }

    /// Checks if the card is an Ace.
    pub fn is_ace(&self) -> bool {
        matches!(self.rank, Rank::Ace)
    }

    /// Checks if the card is a face card (Jack, Queen, King).
    pub fn is_face_card(&self) -> bool {
        matches!(self.rank, Rank::Jack | Rank::Queen | Rank::King)
    }

    /// Checks if the card is a value card (2-10).
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

    /// Checks if the card is a Joker.
    pub fn is_joker(&self) -> bool {
        matches!(self.rank, Rank::Joker)
    }

    /// Compares the rank of this card with another card.
    pub fn is_same_rank(&self, other: &Card) -> bool {
        self.rank == other.rank
    }

    /// Compares the suit of this card with another card.
    pub fn is_same_suit(&self, other: &Card) -> bool {
        self.suit == other.suit
    }

    /// Compares the color of this card with another card.
    pub fn is_same_color(&self, other: &Card) -> bool {
        self.color() == other.color()
    }

    /// Returns a CSV representation of the card.
    pub fn as_csv_row(&self) -> String {
        format!("{},{}", self.rank, self.suit)
    }

    /// Creates a Card from a CSV row.
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

impl From<Card> for u8 {
    fn from(card: Card) -> Self {
        let suit_value = card.suit.value();
        let rank_value = card.rank.value();
        suit_value * 14 + rank_value
    }
}

impl From<Card> for i8 {
    fn from(card: Card) -> Self {
        u8::from(card) as i8
    }
}

impl From<Card> for u16 {
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for i16 {
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for u32 {
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for i32 {
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for u64 {
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for i64 {
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for usize {
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl From<Card> for isize {
    fn from(card: Card) -> Self {
        Self::from(u8::from(card))
    }
}

impl TryFrom<u8> for Card {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }

        let suit = Suit::ALL[(value / 14) as usize];
        let rank = Rank::ALL[(value % 14) as usize];

        Ok(Card::new(suit, rank))
    }
}

impl TryFrom<i8> for Card {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value < 0 || value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<u16> for Card {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<i16> for Card {
    type Error = &'static str;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value < 0 || value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<u32> for Card {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<i32> for Card {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 || value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<u64> for Card {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<i64> for Card {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 || value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<usize> for Card {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl TryFrom<isize> for Card {
    type Error = &'static str;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 || value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }
        Self::try_from(value as u8)
    }
}

impl FromStr for Card {
    type Err = String;

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
