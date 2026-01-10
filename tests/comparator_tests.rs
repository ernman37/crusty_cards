use crusty_cards::{
    AceLowComparator, BridgeComparator, Card, CardComparator, Rank, StandardComparator, Suit,
    TrumpComparator,
};
use std::cmp::Ordering;

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
    assert_eq!(cmp.rank_value(Rank::Two), 0);
    assert_eq!(cmp.rank_value(Rank::Three), 1);
    assert_eq!(cmp.rank_value(Rank::Four), 2);
    assert_eq!(cmp.rank_value(Rank::Five), 3);
    assert_eq!(cmp.rank_value(Rank::Six), 4);
    assert_eq!(cmp.rank_value(Rank::Seven), 5);
    assert_eq!(cmp.rank_value(Rank::Eight), 6);
    assert_eq!(cmp.rank_value(Rank::Nine), 7);
    assert_eq!(cmp.rank_value(Rank::Ten), 8);
    assert_eq!(cmp.rank_value(Rank::Jack), 9);
    assert_eq!(cmp.rank_value(Rank::Queen), 10);
    assert_eq!(cmp.rank_value(Rank::King), 11);
    assert_eq!(cmp.rank_value(Rank::Ace), 12);
    assert_eq!(cmp.rank_value(Rank::Joker), 13);
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
