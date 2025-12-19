
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color{
    Red,
    Black,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Suit{
    Hearts(Color::Red),
    Diamonds(Color::Red),
    Clubs(Color::Black),
    Spades(Color::Black),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card{
    suit: Suit,
    rank: Rank,
}

impl Card{
    fn new(suit: Suit, rank: Rank) -> Self{
        Card { suit, rank }
    }

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

    fn color(&self) -> &Color{
        self.suit.color()
    }

    fn suit(&self) -> &Suit{
        &self.suit
    }

    fn rank(&self) -> &Rank{
        &self.rank
    }

    fn is_ace(&self) -> bool{
        matches!(self.rank, Rank::Ace)
    }

    fn is_face_card(&self) -> bool{
        matches!(self.rank, Rank::Jack | Rank::Queen | Rank::King)
    }

    fn is_value_card(&self) -> bool{
        matches!(self.rank, Rank::Two | Rank::Three | Rank::Four | Rank::Five | Rank::Six | Rank::Seven | Rank::Eight | Rank::Nine | Rank::Ten)
    }

    fn is_joker(&self) -> bool{
        matches!(self.rank, Rank::Joker)
    }
}
