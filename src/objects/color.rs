use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the color of a playing card.
///
/// In a standard deck:
/// - **Red**: Hearts (♥) and Diamonds (♦)
/// - **Black**: Clubs (♣) and Spades (♠)
///
/// Color is useful for games like Euchre where card color affects gameplay,
/// or for display purposes.
///
/// # Examples
///
/// ```rust
/// use crusty_cards::{Card, Suit, Rank, Color};
///
/// let card = Card::new(Suit::Hearts, Rank::Queen);
/// assert_eq!(card.color(), Color::Red);
///
/// let card = Card::new(Suit::Spades, Rank::Ace);
/// assert_eq!(card.color(), Color::Black);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Color {
    Red,
    Black,
}

impl fmt::Display for Color {
    /// Formats the color as a string.
    /// "R" for Red, "B" for Black.
    ///
    /// # Examples
    ///
    /// ```
    /// use crusty_cards::Color;
    /// let red = Color::Red;
    /// let black = Color::Black;
    /// assert_eq!(red.to_string(), "R");
    /// assert_eq!(black.to_string(), "B");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "R"),
            Color::Black => write!(f, "B"),
        }
    }
}
