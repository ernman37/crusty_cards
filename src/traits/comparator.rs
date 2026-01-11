use crate::{Card, Rank, Suit};
use std::cmp::Ordering;

/// Trait for defining custom card ordering rules.
///
/// Different card games have different ranking systems:
/// - Poker: Ace high
/// - Some games: Ace low
/// - Euchre: Jack of trump is highest
/// - Bridge: Complex trump rules
///
/// Implement this trait to define custom ordering for your game.
///
/// # Example
/// ```
/// use crusty_cards::{Card, Rank, Suit};
/// use crusty_cards::CardComparator;
///
/// struct AceLowComparator;
///
/// impl CardComparator for AceLowComparator {
///     fn rank_value(&self, rank: Rank) -> i32 {
///         match rank {
///             Rank::Ace => 1,
///             Rank::Two => 2,
///             // ... etc
///             _ => rank.value() as i32,
///         }
///     }
/// }
/// ```
pub trait CardComparator {
    /// Returns the value of a rank for comparison purposes.
    fn rank_value(&self, rank: Rank) -> i32;

    /// Returns the value of a suit for comparison purposes.
    /// Default implementation returns 0 for all suits (suits are equal).
    fn suit_value(&self, _suit: Suit) -> i32 {
        0
    }

    /// Compares two cards according to this comparator's rules.
    /// Default implementation compares by rank first, then by suit.
    fn compare(&self, a: &Card, b: &Card) -> Ordering {
        let rank_cmp = self.rank_value(a.rank()).cmp(&self.rank_value(b.rank()));
        if rank_cmp != Ordering::Equal {
            rank_cmp
        } else {
            self.suit_value(a.suit()).cmp(&self.suit_value(b.suit()))
        }
    }

    /// Returns true if card `a` is greater than card `b`.
    fn is_greater(&self, a: &Card, b: &Card) -> bool {
        self.compare(a, b) == Ordering::Greater
    }

    /// Returns true if card `a` is less than card `b`.
    fn is_less(&self, a: &Card, b: &Card) -> bool {
        self.compare(a, b) == Ordering::Less
    }

    /// Returns the higher card, or `a` if equal.
    fn max<'a>(&self, a: &'a Card, b: &'a Card) -> &'a Card {
        if self.compare(a, b) != Ordering::Less {
            a
        } else {
            b
        }
    }

    /// Returns the lower card, or `a` if equal.
    fn min<'a>(&self, a: &'a Card, b: &'a Card) -> &'a Card {
        if self.compare(a, b) != Ordering::Greater {
            a
        } else {
            b
        }
    }
}

/// Standard comparator with Ace high (14) and Joker highest (15).
/// Suits are considered equal.
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardComparator;

impl CardComparator for StandardComparator {
    fn rank_value(&self, rank: Rank) -> i32 {
        rank.value() as i32
    }
}

/// Comparator with Ace low (1).
/// Useful for games like Razz or some forms of lowball poker.
#[derive(Debug, Clone, Copy, Default)]
pub struct AceLowComparator;

impl CardComparator for AceLowComparator {
    fn rank_value(&self, rank: Rank) -> i32 {
        match rank {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Joker => 14,
        }
    }
}

/// Comparator that also considers suit order.
/// Default suit order: Spades > Hearts > Diamonds > Clubs (Bridge order).
#[derive(Debug, Clone, Copy, Default)]
pub struct BridgeComparator;

impl CardComparator for BridgeComparator {
    fn rank_value(&self, rank: Rank) -> i32 {
        rank.value() as i32
    }

    fn suit_value(&self, suit: Suit) -> i32 {
        match suit {
            Suit::Clubs => 1,
            Suit::Diamonds => 2,
            Suit::Hearts => 3,
            Suit::Spades => 4,
        }
    }
}

/// A comparator with a trump suit. Trump cards are always higher than non-trump.
#[derive(Debug, Clone, Copy)]
pub struct TrumpComparator {
    trump: Suit,
}

impl TrumpComparator {
    pub fn new(trump: Suit) -> Self {
        Self { trump }
    }

    pub fn trump(&self) -> Suit {
        self.trump
    }
}

impl CardComparator for TrumpComparator {
    fn rank_value(&self, rank: Rank) -> i32 {
        rank.value() as i32
    }

    fn compare(&self, a: &Card, b: &Card) -> Ordering {
        let a_is_trump = a.suit() == self.trump;
        let b_is_trump = b.suit() == self.trump;

        match (a_is_trump, b_is_trump) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => {
                // Both trump or both non-trump: compare by rank
                self.rank_value(a.rank()).cmp(&self.rank_value(b.rank()))
            }
        }
    }
}
