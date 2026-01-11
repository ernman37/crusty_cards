use crusty_cards::{Color, Suit};
use std::collections::HashSet;
use std::str::FromStr;

#[test]
fn test_suit_color() {
    assert_eq!(Suit::Hearts.color(), Color::Red);
    assert_eq!(Suit::Diamonds.color(), Color::Red);
    assert_eq!(Suit::Clubs.color(), Color::Black);
    assert_eq!(Suit::Spades.color(), Color::Black);
}

#[test]
fn test_suit_symbol() {
    assert_eq!(Suit::Hearts.symbol(), "♥");
    assert_eq!(Suit::Diamonds.symbol(), "♦");
    assert_eq!(Suit::Clubs.symbol(), "♣");
    assert_eq!(Suit::Spades.symbol(), "♠");
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
fn test_suit_ordering() {
    assert!(Suit::Hearts < Suit::Diamonds);
    assert!(Suit::Diamonds < Suit::Clubs);
    assert!(Suit::Clubs < Suit::Spades);
}

#[test]
fn test_suit_sorting() {
    let mut suits = vec![Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
    suits.sort();
    assert_eq!(
        suits,
        vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    );
}

#[test]
fn test_suit_hash_in_hashset() {
    let mut set: HashSet<Suit> = HashSet::new();
    for suit in Suit::ALL {
        assert!(set.insert(suit));
    }
    assert_eq!(set.len(), 4);

    // Duplicates should not increase size
    for suit in Suit::ALL {
        assert!(!set.insert(suit));
    }
    assert_eq!(set.len(), 4);
}

#[test]
fn test_suit_all_constant() {
    assert_eq!(Suit::ALL.len(), 4);
    assert!(Suit::ALL.contains(&Suit::Hearts));
    assert!(Suit::ALL.contains(&Suit::Diamonds));
    assert!(Suit::ALL.contains(&Suit::Clubs));
    assert!(Suit::ALL.contains(&Suit::Spades));
}

#[test]
fn test_suit_red_constant() {
    assert_eq!(Suit::RED.len(), 2);
    assert!(Suit::RED.contains(&Suit::Hearts));
    assert!(Suit::RED.contains(&Suit::Diamonds));
}

#[test]
fn test_suit_black_constant() {
    assert_eq!(Suit::BLACK.len(), 2);
    assert!(Suit::BLACK.contains(&Suit::Clubs));
    assert!(Suit::BLACK.contains(&Suit::Spades));
}

#[test]
fn test_suit_value() {
    assert_eq!(Suit::Hearts.value(), 0);
    assert_eq!(Suit::Diamonds.value(), 1);
    assert_eq!(Suit::Clubs.value(), 2);
    assert_eq!(Suit::Spades.value(), 3);
}

#[test]
fn test_suit_from_str() {
    // Hearts
    assert_eq!(Suit::from_str("Hearts"), Ok(Suit::Hearts));
    assert_eq!(Suit::from_str("h"), Ok(Suit::Hearts));
    assert_eq!(Suit::from_str("♥"), Ok(Suit::Hearts));
    // Diamonds
    assert_eq!(Suit::from_str("D"), Ok(Suit::Diamonds));
    assert_eq!(Suit::from_str("diamonds"), Ok(Suit::Diamonds));
    assert_eq!(Suit::from_str("♦"), Ok(Suit::Diamonds));
    // Clubs
    assert_eq!(Suit::from_str("C"), Ok(Suit::Clubs));
    assert_eq!(Suit::from_str("♣"), Ok(Suit::Clubs));
    assert_eq!(Suit::from_str("clubs"), Ok(Suit::Clubs));
    // Spades
    assert_eq!(Suit::from_str("S"), Ok(Suit::Spades));
    assert_eq!(Suit::from_str("spades"), Ok(Suit::Spades));
    assert_eq!(Suit::from_str("♠"), Ok(Suit::Spades));
    // Invalid suit
    assert_eq!(
        Suit::from_str("X"),
        Err("Invalid suit string: X".to_string())
    );
}
