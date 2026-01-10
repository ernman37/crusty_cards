use crusty_cards::Rank;
use std::collections::HashSet;

#[test]
fn test_rank_symbol() {
    assert_eq!(Rank::Two.symbol(), "2");
    assert_eq!(Rank::Three.symbol(), "3");
    assert_eq!(Rank::Four.symbol(), "4");
    assert_eq!(Rank::Five.symbol(), "5");
    assert_eq!(Rank::Six.symbol(), "6");
    assert_eq!(Rank::Seven.symbol(), "7");
    assert_eq!(Rank::Eight.symbol(), "8");
    assert_eq!(Rank::Nine.symbol(), "9");
    assert_eq!(Rank::Ten.symbol(), "T");
    assert_eq!(Rank::Jack.symbol(), "J");
    assert_eq!(Rank::Queen.symbol(), "Q");
    assert_eq!(Rank::King.symbol(), "K");
    assert_eq!(Rank::Ace.symbol(), "A");
    assert_eq!(Rank::Joker.symbol(), "U");
}

#[test]
fn test_rank_value() {
    assert_eq!(Rank::Two.value(), 2);
    assert_eq!(Rank::Three.value(), 3);
    assert_eq!(Rank::Four.value(), 4);
    assert_eq!(Rank::Five.value(), 5);
    assert_eq!(Rank::Six.value(), 6);
    assert_eq!(Rank::Seven.value(), 7);
    assert_eq!(Rank::Eight.value(), 8);
    assert_eq!(Rank::Nine.value(), 9);
    assert_eq!(Rank::Ten.value(), 10);
    assert_eq!(Rank::Jack.value(), 11);
    assert_eq!(Rank::Queen.value(), 12);
    assert_eq!(Rank::King.value(), 13);
    assert_eq!(Rank::Ace.value(), 14);
    assert_eq!(Rank::Joker.value(), 15);
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
fn test_rank_ordering() {
    assert!(Rank::Ace > Rank::King);
    assert!(Rank::King > Rank::Queen);
    assert!(Rank::Queen > Rank::Jack);
    assert!(Rank::Jack > Rank::Ten);
    assert!(Rank::Ten > Rank::Two);
    assert!(Rank::Joker > Rank::Ace);
}

#[test]
fn test_rank_sorting() {
    let mut ranks = vec![Rank::King, Rank::Two, Rank::Ace, Rank::Seven];
    ranks.sort();
    assert_eq!(ranks, vec![Rank::Two, Rank::Seven, Rank::King, Rank::Ace]);
}

#[test]
fn test_rank_hash_in_hashset() {
    let mut set: HashSet<Rank> = HashSet::new();
    for rank in Rank::ALL {
        assert!(set.insert(rank));
    }
    assert_eq!(set.len(), 14);

    // Duplicates should not increase size
    for rank in Rank::ALL {
        assert!(!set.insert(rank));
    }
    assert_eq!(set.len(), 14);
}

#[test]
fn test_rank_all_constant() {
    assert_eq!(Rank::ALL.len(), 14);
    assert!(Rank::ALL.contains(&Rank::Two));
    assert!(Rank::ALL.contains(&Rank::Ace));
    assert!(Rank::ALL.contains(&Rank::Joker));
}

#[test]
fn test_rank_standard_constant() {
    assert_eq!(Rank::STANDARD.len(), 13);
    assert!(Rank::STANDARD.contains(&Rank::Two));
    assert!(Rank::STANDARD.contains(&Rank::Ace));
    assert!(!Rank::STANDARD.contains(&Rank::Joker));
}
