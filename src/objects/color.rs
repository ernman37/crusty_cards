use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the color of a card. Useful for games that utilize card colors (e.g., Euchre)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Color {
    Red,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Black => write!(f, "Black"),
        }
    }
}
