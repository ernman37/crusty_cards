
/// Represents the color of a card. Useful for games that utilize card colors (e.g., Euchre)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color{
    Red,
    Black,
}
/// Represents the suit of a card.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit{
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit{
    pub fn color(&self) -> &Color{
        match self{
            Suit::Hearts => &Color::Red,
            Suit::Diamonds => &Color::Red,
            Suit::Clubs => &Color::Black,
            Suit::Spades => &Color::Black,
        }
    }

    pub fn is_black(&self) -> bool{
        matches!(self, Suit::Clubs | Suit::Spades)
    }

    pub fn is_red(&self) -> bool{
        matches!(self, Suit::Hearts | Suit::Diamonds)
    }
}

/// Represents the rank of a card. Useful for games that utilize card ranks (e.g., Poker)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rank{
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

/// Represents a playing card with a suit and rank.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card{
    suit: Suit,
    rank: Rank,
}

impl Card{
    /// Creates a new card with the given suit and rank.
    pub fn new(suit: Suit, rank: Rank) -> Self{
        Card { suit, rank }
    }

    /// Displays the card in a human-readable format.
    pub fn display(&self) -> String{
        let rank_str = self.get_value_str();
        let suit_str = self.get_suit_str();

        format!("{}{}", rank_str, suit_str)
    }

    fn get_value_str(&self) -> &str{
        match self.rank{
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
            Rank::Joker => "U",
        }
    }

    fn get_suit_str(&self) -> &str{
        match self.suit{
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }

    /// Returns the color of the card.
    pub fn color(&self) -> &Color{
        self.suit.color()
    }

    /// Returns the suit of the card.
    pub fn suit(&self) -> &Suit{
        &self.suit
    }

    /// Returns the rank of the card.
    pub fn rank(&self) -> &Rank{
        &self.rank
    }

    /// Checks if the card is an Ace.
    pub fn is_ace(&self) -> bool{
        matches!(self.rank, Rank::Ace)
    }

    /// Checks if the card is a face card (Jack, Queen, King).
    pub fn is_face_card(&self) -> bool{
        matches!(self.rank, Rank::Jack | Rank::Queen | Rank::King)
    }

    /// Checks if the card is a value card (2-10).
    pub fn is_value_card(&self) -> bool{
        matches!(self.rank, Rank::Two | Rank::Three | Rank::Four | Rank::Five | Rank::Six | Rank::Seven | Rank::Eight | Rank::Nine | Rank::Ten)
    }

    /// Checks if the card is a Joker.
    pub fn is_joker(&self) -> bool{
        matches!(self.rank, Rank::Joker)
    }
}
