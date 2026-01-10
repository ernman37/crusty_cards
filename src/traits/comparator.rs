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
        if self.compare(a, b) != Ordering::Less { a } else { b }
    }

    /// Returns the lower card, or `a` if equal.
    fn min<'a>(&self, a: &'a Card, b: &'a Card) -> &'a Card {
        if self.compare(a, b) != Ordering::Greater { a } else { b }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Card, Rank, Suit};

    #[test]
    fn test_standard_comparator() {
        let cmp = StandardComparator;
        let ace = Card::new(Suit::Spades, Rank::Ace);
        let king = Card::new(Suit::Hearts, Rank::King);
        let two = Card::new(Suit::Clubs, Rank::Two);

        assert!(cmp.is_greater(&ace, &king));
        assert!(cmp.is_greater(&king, &two));
        assert!(cmp.is_less(&two, &ace));
        assert_eq!(cmp.compare(&ace, &ace), Ordering::Equal);
    }

    #[test]
    fn test_standard_comparator_rank_values() {
        let cmp = StandardComparator;
        assert_eq!(cmp.rank_value(Rank::Two), 2);
        assert_eq!(cmp.rank_value(Rank::Ten), 10);
        assert_eq!(cmp.rank_value(Rank::Jack), 11);
        assert_eq!(cmp.rank_value(Rank::Queen), 12);
        assert_eq!(cmp.rank_value(Rank::King), 13);
        assert_eq!(cmp.rank_value(Rank::Ace), 14);
        assert_eq!(cmp.rank_value(Rank::Joker), 15);
    }

    #[test]
    fn test_ace_low_comparator() {
        let cmp = AceLowComparator;
        let ace = Card::new(Suit::Spades, Rank::Ace);
        let two = Card::new(Suit::Hearts, Rank::Two);
        let king = Card::new(Suit::Clubs, Rank::King);

        assert!(cmp.is_less(&ace, &two));
        assert!(cmp.is_less(&ace, &king));
        assert!(cmp.is_greater(&king, &ace));
        assert_eq!(cmp.rank_value(Rank::Ace), 1);
    }

    #[test]
    fn test_bridge_comparator_suit_order() {
        let cmp = BridgeComparator;

        // Same rank, different suits
        let ace_spades = Card::new(Suit::Spades, Rank::Ace);
        let ace_hearts = Card::new(Suit::Hearts, Rank::Ace);
        let ace_diamonds = Card::new(Suit::Diamonds, Rank::Ace);
        let ace_clubs = Card::new(Suit::Clubs, Rank::Ace);

        assert!(cmp.is_greater(&ace_spades, &ace_hearts));
        assert!(cmp.is_greater(&ace_hearts, &ace_diamonds));
        assert!(cmp.is_greater(&ace_diamonds, &ace_clubs));
    }

    #[test]
    fn test_trump_comparator() {
        let cmp = TrumpComparator::new(Suit::Hearts);

        // Trump beats non-trump
        let two_hearts = Card::new(Suit::Hearts, Rank::Two);
        let ace_spades = Card::new(Suit::Spades, Rank::Ace);

        assert!(cmp.is_greater(&two_hearts, &ace_spades));

        // Within trump, rank matters
        let ace_hearts = Card::new(Suit::Hearts, Rank::Ace);
        assert!(cmp.is_greater(&ace_hearts, &two_hearts));

        // Non-trump vs non-trump, rank matters
        let king_clubs = Card::new(Suit::Clubs, Rank::King);
        assert!(cmp.is_greater(&ace_spades, &king_clubs));
    }

    #[test]
    fn test_comparator_max_min() {
        let cmp = StandardComparator;
        let ace = Card::new(Suit::Spades, Rank::Ace);
        let two = Card::new(Suit::Hearts, Rank::Two);

        assert_eq!(cmp.max(&ace, &two), &ace);
        assert_eq!(cmp.min(&ace, &two), &two);
        assert_eq!(cmp.max(&ace, &ace), &ace);
    }

    #[test]
    fn test_sorting_with_comparator() {
        let cmp = StandardComparator;
        let mut cards = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Clubs, Rank::Ace),
        ];

        cards.sort_by(|a, b| cmp.compare(a, b));

        assert_eq!(cards[0].rank(), Rank::Two);
        assert_eq!(cards[1].rank(), Rank::King);
        assert_eq!(cards[2].rank(), Rank::Ace);
    }

    #[test]
    fn test_sorting_with_ace_low() {
        let cmp = AceLowComparator;
        let mut cards = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Clubs, Rank::Ace),
        ];

        cards.sort_by(|a, b| cmp.compare(a, b));

        assert_eq!(cards[0].rank(), Rank::Ace);
        assert_eq!(cards[1].rank(), Rank::Two);
        assert_eq!(cards[2].rank(), Rank::King);
    }
}
