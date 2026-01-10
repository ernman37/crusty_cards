use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use super::color::Color;

/// Represents the suit of a card.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    /// All four suits in order.
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

    /// Red suits (Hearts, Diamonds).
    pub const RED: [Suit; 2] = [Suit::Hearts, Suit::Diamonds];

    /// Black suits (Clubs, Spades).
    pub const BLACK: [Suit; 2] = [Suit::Clubs, Suit::Spades];

    /// Returns the color of the suit.
    pub const fn color(&self) -> Color {
        match self {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            Suit::Clubs | Suit::Spades => Color::Black,
        }
    }

    /// Returns the Unicode symbol for the suit.
    pub const fn symbol(&self) -> &str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }

    /// Returns integer value for the suit
    pub const fn value(&self) -> u8 {
        match self {
            Suit::Hearts => 0,
            Suit::Diamonds => 1,
            Suit::Clubs => 2,
            Suit::Spades => 3,
        }
    }

}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hearts" | "h" | "♥" => Ok(Suit::Hearts),
            "diamonds" | "d" | "♦" => Ok(Suit::Diamonds),
            "clubs" | "c" | "♣" => Ok(Suit::Clubs),
            "spades" | "s" | "♠" => Ok(Suit::Spades),
            _ => Err(format!("Invalid suit string: {}", s)),
        }
    }
}
