
/// Represents the color of a card. Useful for games that utilize card colors (e.g., Euchre)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color{
    Red,
    Black,
}

/// Represents the suit of a card.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Suit{
    Hearts(Color::Red),
    Diamonds(Color::Red),
    Clubs(Color::Black),
    Spades(Color::Black),
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
enum Rank{
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
    fn new(suit: Suit, rank: Rank) -> Self{
        Card { suit, rank }
    }

    /// Displays the card in a human-readable format.
    fn display(&self) -> String{
        let rank_str = match self.rank{
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
        };

        let suit_str = match self.suit{
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };

        format!("{}{}", rank_str, suit_str)
    }

    /// Returns the color of the card.
    fn color(&self) -> &Color{
        self.suit.color()
    }

    /// Returns the suit of the card.
    fn suit(&self) -> &Suit{
        &self.suit
    }

    /// Returns the rank of the card.
    fn rank(&self) -> &Rank{
        &self.rank
    }

    /// Checks if the card is an Ace.
    fn is_ace(&self) -> bool{
        matches!(self.rank, Rank::Ace)
    }

    /// Checks if the card is a face card (Jack, Queen, King).
    fn is_face_card(&self) -> bool{
        matches!(self.rank, Rank::Jack | Rank::Queen | Rank::King)
    }

    /// Checks if the card is a value card (2-10).
    fn is_value_card(&self) -> bool{
        matches!(self.rank, Rank::Two | Rank::Three | Rank::Four | Rank::Five | Rank::Six | Rank::Seven | Rank::Eight | Rank::Nine | Rank::Ten)
    }

    /// Checks if the card is a Joker.
    fn is_joker(&self) -> bool{
        matches!(self.rank, Rank::Joker)
    }
}
