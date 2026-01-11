use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use super::color::Color;

/// Represents the four suits in a standard deck of playing cards.
///
/// Suits are ordered as: Hearts, Diamonds, Clubs, Spades (by their enum discriminant).
/// For custom ordering in games, use a [`CardComparator`](crate::CardComparator).
///
/// # Examples
///
/// ```rust
/// use crusty_cards::Suit;
///
/// // Access all suits
/// for suit in Suit::ALL {
///     println!("{}", suit);  // ♥ ♦ ♣ ♠
/// }
///
/// // Check color
/// assert_eq!(Suit::Hearts.color(), crusty_cards::Color::Red);
/// assert_eq!(Suit::Spades.color(), crusty_cards::Color::Black);
///
/// // Parse from string
/// let suit: Suit = "♠".parse().unwrap();
/// let suit: Suit = "SPADES".parse().unwrap();
/// let suit: Suit = "S".parse().unwrap();
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Suit {
    /// ♥ - Red suit
    Hearts,
    /// ♦ - Red suit
    Diamonds,
    /// ♣ - Black suit
    Clubs,
    /// ♠ - Black suit
    Spades,
}

impl Suit {
    /// All four suits in order: Hearts, Diamonds, Clubs, Spades.
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

    /// The two red suits: Hearts and Diamonds.
    pub const RED: [Suit; 2] = [Suit::Hearts, Suit::Diamonds];

    /// The two black suits: Clubs and Spades.
    pub const BLACK: [Suit; 2] = [Suit::Clubs, Suit::Spades];

    /// Returns the color of the suit.
    ///
    /// - Hearts and Diamonds → [`Color::Red`]
    /// - Clubs and Spades → [`Color::Black`]
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::{Suit, Color};
    /// let suit = Suit::Hearts;
    /// assert_eq!(suit.color(), Color::Red);
    /// ```
    pub const fn color(&self) -> Color {
        match self {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            Suit::Clubs | Suit::Spades => Color::Black,
        }
    }

    /// Returns the Unicode symbol for the suit.
    ///
    /// # Returns
    /// | Suit     | Symbol |
    /// |----------|--------|
    /// | Hearts   | ♥      |
    /// | Diamonds | ♦      |
    /// | Clubs    | ♣      |
    /// | Spades   | ♠      |
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Suit;
    /// let suit = Suit::Hearts;
    /// assert_eq!(suit.symbol(), "♥");
    /// ```
    pub const fn symbol(&self) -> &str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }

    /// Returns a numeric index for the suit (0-3).
    ///
    /// Useful for compact card representations or array indexing.
    ///
    /// # Returns
    /// | Suit     | Value |
    /// |----------|-------|
    /// | Hearts   | 0     |
    /// | Diamonds | 1     |
    /// | Clubs    | 2     |
    /// | Spades   | 3     |
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Suit;
    /// let suit = Suit::Hearts;
    /// assert_eq!(suit.value(), 0);
    /// ```
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
    /// Formats the suit as a string.
    /// Utilizes the `symbol()` method for representation.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Suit;
    /// let suit = Suit::Hearts;
    /// assert_eq!(suit.to_string(), "♥");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl FromStr for Suit {
    type Err = String;

    /// Parses a suit from a string.
    ///
    /// Accepts:
    /// - Full names: "HEARTS", "DIAMONDS", "CLUBS", "SPADES" (case-insensitive)
    /// - Single letters: "H", "D", "C", "S"
    /// - Unicode symbols: "♥", "♦", "♣", "♠"
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::Suit;
    /// let suit1: Suit = "Hearts".parse().unwrap();
    /// let suit2: Suit = "d".parse().unwrap();
    /// let suit3: Suit = "♣".parse().unwrap();
    /// let suit4: Suit = "spades".parse().unwrap();
    /// assert_eq!(suit1, Suit::Hearts);
    /// assert_eq!(suit2, Suit::Diamonds);
    /// assert_eq!(suit3, Suit::Clubs);
    /// assert_eq!(suit4, Suit::Spades);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "HEARTS" | "H" | "♥" => Ok(Suit::Hearts),
            "DIAMONDS" | "D" | "♦" => Ok(Suit::Diamonds),
            "CLUBS" | "C" | "♣" => Ok(Suit::Clubs),
            "SPADES" | "S" | "♠" => Ok(Suit::Spades),
            _ => Err(format!("Invalid suit string: {}", s)),
        }
    }
}
