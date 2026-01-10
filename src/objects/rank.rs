use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// Represents the rank of a card. Useful for games that utilize card ranks (e.g., Poker)
///
/// Note: `Ord` is implemented with Ace high (14) and Joker highest (15).
/// For custom ordering (e.g., Ace low), use a `CardComparator`.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

impl Rank {
    /// All 14 ranks including Joker.
    pub const ALL: [Rank; 14] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
        Rank::Joker,
    ];

    /// Standard 13 ranks (no Joker).
    pub const STANDARD: [Rank; 13] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    /// Returns the single-character symbol for the rank.
    pub const fn symbol(&self) -> &str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
            Rank::Joker => "U",
        }
    }

    /// Returns the standard value of the rank for ordering purposes.
    /// Ace is high (14), Joker is highest (15).
    /// For custom values, use a `CardComparator`.
    pub const fn value(&self) -> u8 {
        match self {
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
        }
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl FromStr for Rank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "2" | "TWO" => Ok(Rank::Two),
            "3" | "THREE" => Ok(Rank::Three),
            "4" | "FOUR" => Ok(Rank::Four),
            "5" | "FIVE" => Ok(Rank::Five),
            "6" | "SIX" => Ok(Rank::Six),
            "7" | "SEVEN" => Ok(Rank::Seven),
            "8" | "EIGHT" => Ok(Rank::Eight),
            "9" | "NINE" => Ok(Rank::Nine),
            "T" | "TEN" | "10" => Ok(Rank::Ten),
            "J" | "JACK" => Ok(Rank::Jack),
            "Q" | "QUEEN" => Ok(Rank::Queen),
            "K" | "KING" => Ok(Rank::King),
            "A" | "ACE" => Ok(Rank::Ace),
            "U" | "JOKER" => Ok(Rank::Joker),
            _ => Err(format!("Invalid rank symbol: {}", s)),
        }
    }
}
