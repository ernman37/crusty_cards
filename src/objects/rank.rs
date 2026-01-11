use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// Represents the rank (value) of a playing card.
///
/// Standard ranks are Two through Ace, with an optional Joker for 54-card decks.
///
/// # Ordering
///
/// The default ordering (`Ord`) treats Ace as high (value 12) and Joker as highest (13).
/// For custom ordering (e.g., Ace low), use a [`CardComparator`](crate::CardComparator).
///
/// # Examples
///
/// ```rust
/// use crusty_cards::Rank;
///
/// // Compare ranks (Ace high by default)
/// assert!(Rank::Ace > Rank::King);
/// assert!(Rank::Two < Rank::Three);
///
/// // Parse from string
/// let rank: Rank = "A".parse().unwrap();
/// let rank: Rank = "ACE".parse().unwrap();
/// let rank: Rank = "10".parse().unwrap();
///
/// // Access all standard ranks
/// for rank in Rank::STANDARD {
///     println!("{}", rank);
/// }
/// ```
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

    /// Standard 13 ranks (Two through Ace, no Joker).
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

    /// Returns the display symbol for the rank.
    ///
    /// # Returns
    /// | Rank  | Symbol |
    /// |-------|--------|
    /// | Two   | 2      |
    /// | Three | 3      |
    /// | ...   | ...    |
    /// | Ten   | T      |
    /// | Jack  | J      |
    /// | Queen | Q      |
    /// | King  | K      |
    /// | Ace   | A      |
    /// | Joker | U      |
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Rank;
    /// let rank = Rank::Two;
    /// assert_eq!(rank.symbol(), "2");
    /// ```
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

    /// Returns the numeric value of the rank for ordering (0-13).
    ///
    /// Ace is high (12), Joker is highest (13).
    /// For custom values, use a [`CardComparator`](crate::CardComparator).
    ///
    /// # Returns
    /// | Rank  | Value |
    /// |-------|-------|
    /// | Two   | 0     |
    /// | Three | 1     |
    /// | ...   | ...   |
    /// | King  | 11    |
    /// | Ace   | 12    |
    /// | Joker | 13    |
    ///
    /// # Examples
    /// ```rust
    /// use crusty_cards::Rank;
    /// let rank = Rank::Two;
    /// assert_eq!(rank.value(), 0);
    /// ```
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
    /// Compares two ranks for ordering.
    /// Ranks are ordered by their numeric value (2-14).
    /// Joker is the highest rank.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Rank;
    /// let rank1 = Rank::Two;
    /// let rank2 = Rank::Three;
    /// assert!(rank1 < rank2);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Rank {
    /// Compares two ranks for partial ordering.
    /// Ranks are ordered by their numeric value (2-14).
    /// Joker is the highest rank.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Rank;
    /// let rank1 = Rank::Two;
    /// let rank2 = Rank::Three;
    /// assert!(rank1 < rank2);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Rank {
    /// Formats the rank as a string.
    /// Utilizes the `symbol()` method for representation.
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Rank;
    /// let rank = Rank::Two;
    /// assert_eq!(rank.to_string(), "2");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl FromStr for Rank {
    type Err = String;

    /// Parses a rank from a string.
    ///
    /// # Accepts:
    /// - Numbers: "2", "3", ..., "10"
    /// - Letters: "T" (Ten), "J", "Q", "K", "A", "U" (Joker)
    /// - Full names: "TWO", "JACK", "ACE", etc. (case-insensitive)
    ///
    /// # Examples
    /// ```
    /// use crusty_cards::Rank;
    /// use std::str::FromStr;
    /// let rank = Rank::from_str("TwO").unwrap();
    /// assert_eq!(rank, Rank::Two);
    /// ```
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
            _ => Err(format!("Invalid rank string: {}", s)),
        }
    }
}
