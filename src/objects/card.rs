use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the color of a card. Useful for games that utilize card colors (e.g., Euchre)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    Red,
    Black,
}

/// Represents the suit of a card.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub const fn color(&self) -> Color {
        match self {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            Suit::Clubs | Suit::Spades => Color::Black,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        write!(f, "{}", symbol)
    }
}

/// Represents the rank of a card. Useful for games that utilize card ranks (e.g., Poker)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
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

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
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
        };
        write!(f, "{}", symbol)
    }
}

/// Represents a playing card with a suit and rank.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
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

    /// Displays the card using ascii art.
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

#[cfg(test)]
mod tests {
    use super::{Card, Color, Rank, Suit};

    #[test]
    fn test_suit_color() {
        let suit = Suit::Hearts;
        assert_eq!(suit.color(), Color::Red);
        let suit = Suit::Diamonds;
        assert_eq!(suit.color(), Color::Red);
        let suit = Suit::Clubs;
        assert_eq!(suit.color(), Color::Black);
        let suit = Suit::Spades;
        assert_eq!(suit.color(), Color::Black);
    }

    #[test]
    fn test_card_color() {
        let heart_card = Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(heart_card.color(), Color::Red);
        assert_ne!(heart_card.color(), Color::Black);

        let diamond_card = Card::new(Suit::Diamonds, Rank::Ace);
        assert_eq!(diamond_card.color(), Color::Red);
        assert_ne!(diamond_card.color(), Color::Black);

        let clubs_card = Card::new(Suit::Clubs, Rank::King);
        assert_eq!(clubs_card.color(), Color::Black);
        assert_ne!(clubs_card.color(), Color::Red);

        let spade_card = Card::new(Suit::Spades, Rank::Queen);
        assert_eq!(spade_card.color(), Color::Black);
        assert_ne!(spade_card.color(), Color::Red);
    }

    #[test]
    fn test_card_is_ace() {
        let ace_card = Card::new(Suit::Hearts, Rank::Ace);
        assert!(ace_card.is_ace());
        assert!(!ace_card.is_face_card());
        assert!(!ace_card.is_value_card());
        assert!(!ace_card.is_joker());

        let non_ace_card = Card::new(Suit::Hearts, Rank::King);
        assert!(!non_ace_card.is_ace());
    }

    #[test]
    fn test_card_is_face_card() {
        let face_card = Card::new(Suit::Spades, Rank::Queen);
        assert!(!face_card.is_ace());
        assert!(face_card.is_face_card());
        assert!(!face_card.is_value_card());
        assert!(!face_card.is_joker());

        let non_face_card = Card::new(Suit::Diamonds, Rank::Ten);
        assert!(!non_face_card.is_face_card());
    }

    #[test]
    fn test_card_is_value_card() {
        let value_card = Card::new(Suit::Clubs, Rank::Seven);
        assert!(!value_card.is_ace());
        assert!(!value_card.is_face_card());
        assert!(value_card.is_value_card());
        assert!(!value_card.is_joker());

        let non_value_card = Card::new(Suit::Hearts, Rank::Jack);
        assert!(!non_value_card.is_value_card());
    }

    #[test]
    fn test_card_is_joker() {
        let joker_card = Card::new(Suit::Hearts, Rank::Joker);
        assert!(!joker_card.is_ace());
        assert!(!joker_card.is_face_card());
        assert!(!joker_card.is_value_card());
        assert!(joker_card.is_joker());

        let non_joker_card = Card::new(Suit::Spades, Rank::Ace);
        assert!(!non_joker_card.is_joker());
    }

    #[test]
    fn test_card_is_equal() {
        let card1 = Card::new(Suit::Hearts, Rank::Ace);
        let card2 = Card::new(Suit::Hearts, Rank::Ace);
        let card3 = Card::new(Suit::Spades, Rank::King);

        assert!(card1 == card2);
        assert!(card2 == card1);
        assert!(card1 != card3);
        assert!(card2 != card3);
    }

    #[test]
    fn test_card_is_same_rank() {
        let card1 = Card::new(Suit::Hearts, Rank::Ace);
        let card2 = Card::new(Suit::Spades, Rank::Ace);
        let card3 = Card::new(Suit::Diamonds, Rank::King);

        assert!(card1.is_same_rank(&card2));
        assert!(card2.is_same_rank(&card1));
        assert!(!card1.is_same_rank(&card3));
        assert!(!card2.is_same_rank(&card3));
    }

    #[test]
    fn test_card_is_same_suit() {
        let card1 = Card::new(Suit::Hearts, Rank::Ace);
        let card2 = Card::new(Suit::Hearts, Rank::King);
        let card3 = Card::new(Suit::Spades, Rank::Ace);

        assert!(card1.is_same_suit(&card2));
        assert!(card2.is_same_suit(&card1));
        assert!(!card1.is_same_suit(&card3));
        assert!(!card2.is_same_suit(&card3));
    }

    #[test]
    fn test_card_is_same_color() {
        let card1 = Card::new(Suit::Hearts, Rank::Ace);
        let card2 = Card::new(Suit::Diamonds, Rank::King);
        let card3 = Card::new(Suit::Spades, Rank::Ace);

        assert!(card1.is_same_color(&card2));
        assert!(card2.is_same_color(&card1));
        assert!(!card1.is_same_color(&card3));
        assert!(!card2.is_same_color(&card3));
    }

    #[test]
    fn test_card_display_ascii() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        let expected = "┌─────┐\n│A   │\n│  ♥  │\n│   A│\n└─────┘";
        assert_eq!(card.display_ascii(), expected);
    }

    #[test]
    fn test_card_serialization() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        let serialized = serde_json::to_string(&card).unwrap();
        let deserialized: Card = serde_json::from_str(&serialized).unwrap();
        assert_eq!(card, deserialized);
    }
}
