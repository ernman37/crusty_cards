use serde::{Deserialize, Serialize};
use std::convert::{From, TryFrom};
use std::fmt;

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
}

impl TryFrom<usize> for Card {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }

        let suit = Suit::ALL[value / 14];
        let rank = Rank::ALL[value % 14];

        Ok(Card::new(suit, rank))
    }
}

impl From<Card> for usize {
    fn from(card: Card) -> Self {
        let suit_value = match card.suit {
            Suit::Hearts => 0,
            Suit::Diamonds => 1,
            Suit::Clubs => 2,
            Suit::Spades => 3,
        };

        let rank_value = match card.rank {
            Rank::Two => 0,
            Rank::Three => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
            Rank::Joker => 13,
        };

        suit_value * 14 + rank_value
    }
}
