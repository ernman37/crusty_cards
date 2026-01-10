use std::convert::{From, TryFrom};
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
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
    pub const RED: [Suit; 2] = [Suit::Hearts, Suit::Diamonds];
    pub const BLACK: [Suit; 2] = [Suit::Clubs, Suit::Spades];

    pub const fn color(&self) -> Color {
        match self {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            Suit::Clubs | Suit::Spades => Color::Black,
        }
    }

    pub const fn symbol(&self) -> &str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = self.symbol();
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

impl Rank {
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
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = self.symbol();
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

impl TryFrom<usize> for Card {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= 56 {
            return Err("Value out of range for standard 56-card deck (including 4 jokers, one of each suit)");
        }

        let suits = [
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Clubs,
            Suit::Spades
        ];
        let ranks = [
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

        let suit = suits[value / 14];
        let rank = ranks[value % 14];

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

    #[test]
    fn test_suit_display() {
        assert_eq!(format!("{}", Suit::Hearts), "♥");
        assert_eq!(format!("{}", Suit::Diamonds), "♦");
        assert_eq!(format!("{}", Suit::Clubs), "♣");
        assert_eq!(format!("{}", Suit::Spades), "♠");
    }

    #[test]
    fn test_suit_to_string() {
        assert_eq!(Suit::Hearts.to_string(), "♥");
        assert_eq!(Suit::Diamonds.to_string(), "♦");
        assert_eq!(Suit::Clubs.to_string(), "♣");
        assert_eq!(Suit::Spades.to_string(), "♠");
    }

    #[test]
    fn test_rank_display() {
        assert_eq!(format!("{}", Rank::Two), "2");
        assert_eq!(format!("{}", Rank::Three), "3");
        assert_eq!(format!("{}", Rank::Four), "4");
        assert_eq!(format!("{}", Rank::Five), "5");
        assert_eq!(format!("{}", Rank::Six), "6");
        assert_eq!(format!("{}", Rank::Seven), "7");
        assert_eq!(format!("{}", Rank::Eight), "8");
        assert_eq!(format!("{}", Rank::Nine), "9");
        assert_eq!(format!("{}", Rank::Ten), "T");
        assert_eq!(format!("{}", Rank::Jack), "J");
        assert_eq!(format!("{}", Rank::Queen), "Q");
        assert_eq!(format!("{}", Rank::King), "K");
        assert_eq!(format!("{}", Rank::Ace), "A");
        assert_eq!(format!("{}", Rank::Joker), "U");
    }

    #[test]
    fn test_rank_to_string() {
        assert_eq!(Rank::Ace.to_string(), "A");
        assert_eq!(Rank::King.to_string(), "K");
        assert_eq!(Rank::Queen.to_string(), "Q");
        assert_eq!(Rank::Jack.to_string(), "J");
        assert_eq!(Rank::Ten.to_string(), "T");
        assert_eq!(Rank::Two.to_string(), "2");
        assert_eq!(Rank::Joker.to_string(), "U");
    }

    #[test]
    fn test_card_display() {
        let ace_of_spades = Card::new(Suit::Spades, Rank::Ace);
        assert_eq!(format!("{}", ace_of_spades), "A♠");

        let king_of_hearts = Card::new(Suit::Hearts, Rank::King);
        assert_eq!(format!("{}", king_of_hearts), "K♥");

        let two_of_clubs = Card::new(Suit::Clubs, Rank::Two);
        assert_eq!(format!("{}", two_of_clubs), "2♣");

        let joker = Card::new(Suit::Diamonds, Rank::Joker);
        assert_eq!(format!("{}", joker), "U♦");
    }

    #[test]
    fn test_card_to_string() {
        let ace_of_spades = Card::new(Suit::Spades, Rank::Ace);
        assert_eq!(ace_of_spades.to_string(), "A♠");

        let queen_of_diamonds = Card::new(Suit::Diamonds, Rank::Queen);
        assert_eq!(queen_of_diamonds.to_string(), "Q♦");
    }

    #[test]
    fn test_card_display_all_suits() {
        let ranks = [Rank::Ace, Rank::King, Rank::Two];
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let suit_symbols = ["♥", "♦", "♣", "♠"];
        let rank_symbols = ["A", "K", "2"];

        for (rank, rank_sym) in ranks.iter().zip(rank_symbols.iter()) {
            for (suit, suit_sym) in suits.iter().zip(suit_symbols.iter()) {
                let card = Card::new(*suit, *rank);
                let expected = format!("{}{}", rank_sym, suit_sym);
                assert_eq!(card.to_string(), expected);
            }
        }
    }

    #[test]
    fn test_display_ascii_all_suits() {
        let card_hearts = Card::new(Suit::Hearts, Rank::Ace);
        assert!(card_hearts.display_ascii().contains("♥"));
        assert!(card_hearts.display_ascii().contains("A"));

        let card_diamonds = Card::new(Suit::Diamonds, Rank::King);
        assert!(card_diamonds.display_ascii().contains("♦"));
        assert!(card_diamonds.display_ascii().contains("K"));

        let card_clubs = Card::new(Suit::Clubs, Rank::Queen);
        assert!(card_clubs.display_ascii().contains("♣"));
        assert!(card_clubs.display_ascii().contains("Q"));

        let card_spades = Card::new(Suit::Spades, Rank::Jack);
        assert!(card_spades.display_ascii().contains("♠"));
        assert!(card_spades.display_ascii().contains("J"));
    }

    #[test]
    fn test_display_ascii_structure() {
        let card = Card::new(Suit::Spades, Rank::King);
        let ascii = card.display_ascii();
        let lines: Vec<&str> = ascii.lines().collect();

        // Should have 5 lines
        assert_eq!(lines.len(), 5);

        // Check top and bottom borders
        assert_eq!(lines[0], "┌─────┐");
        assert_eq!(lines[4], "└─────┘");

        // Check that rank appears twice (top-left and bottom-right)
        assert!(lines[1].contains("K"));
        assert!(lines[3].contains("K"));

        // Check that suit appears in the middle
        assert!(lines[2].contains("♠"));
    }

    #[test]
    fn test_card_try_from_usize() {
        let card = Card::try_from(0).unwrap();
        assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
        let card = Card::try_from(13).unwrap();
        assert_eq!(card, Card::new(Suit::Hearts, Rank::Joker));
        let card = Card::try_from(14).unwrap();
        assert_eq!(card, Card::new(Suit::Diamonds, Rank::Two));
        let card = Card::try_from(27).unwrap();
        assert_eq!(card, Card::new(Suit::Diamonds, Rank::Joker));
        let card = Card::try_from(28).unwrap();
        assert_eq!(card, Card::new(Suit::Clubs, Rank::Two));
        let card = Card::try_from(41).unwrap();
        assert_eq!(card, Card::new(Suit::Clubs, Rank::Joker));
        let card = Card::try_from(42).unwrap();
        assert_eq!(card, Card::new(Suit::Spades, Rank::Two));
        let card = Card::try_from(55).unwrap();
        assert_eq!(card, Card::new(Suit::Spades, Rank::Joker));
    }

    #[test]
    fn test_card_try_from_usize_out_of_range() {
        let result = Card::try_from(56);
        assert!(result.is_err());
    }

    #[test]
    fn test_usize_from_card() {
        let card = Card::new(Suit::Hearts, Rank::Two);
        assert_eq!(usize::from(card), 0);
        let card = Card::new(Suit::Hearts, Rank::Joker);
        assert_eq!(usize::from(card), 13);
        let card = Card::new(Suit::Diamonds, Rank::Two);
        assert_eq!(usize::from(card), 14);
        let card = Card::new(Suit::Diamonds, Rank::Joker);
        assert_eq!(usize::from(card), 27);
        let card = Card::new(Suit::Clubs, Rank::Two);
        assert_eq!(usize::from(card), 28);
        let card = Card::new(Suit::Clubs, Rank::Joker);
        assert_eq!(usize::from(card), 41);
        let card = Card::new(Suit::Spades, Rank::Two);
        assert_eq!(usize::from(card), 42);
        let card = Card::new(Suit::Spades, Rank::Joker);
        assert_eq!(usize::from(card), 55);
    }

    #[test]
    fn test_card_int_trip() {
        for i in 0..56 {
            let card = Card::try_from(i).unwrap();
            let j = usize::from(card);
            assert_eq!(i, j);
        }
    }
}
