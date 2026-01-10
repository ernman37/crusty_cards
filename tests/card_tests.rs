use crusty_cards::{Card, Rank, Suit};
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

#[test]
fn test_card_new() {
    let card = Card::new(Suit::Hearts, Rank::Ace);
    assert_eq!(card.suit(), Suit::Hearts);
    assert_eq!(card.rank(), Rank::Ace);
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

    assert_eq!(card1, card2);
    assert_ne!(card1, card3);
}

#[test]
fn test_card_is_same_rank() {
    let card1 = Card::new(Suit::Hearts, Rank::Ace);
    let card2 = Card::new(Suit::Spades, Rank::Ace);
    let card3 = Card::new(Suit::Diamonds, Rank::King);

    assert!(card1.is_same_rank(&card2));
    assert!(!card1.is_same_rank(&card3));
}

#[test]
fn test_card_is_same_suit() {
    let card1 = Card::new(Suit::Hearts, Rank::Ace);
    let card2 = Card::new(Suit::Hearts, Rank::King);
    let card3 = Card::new(Suit::Spades, Rank::Ace);

    assert!(card1.is_same_suit(&card2));
    assert!(!card1.is_same_suit(&card3));
}

#[test]
fn test_card_is_same_color() {
    let card1 = Card::new(Suit::Hearts, Rank::Ace);
    let card2 = Card::new(Suit::Diamonds, Rank::King);
    let card3 = Card::new(Suit::Spades, Rank::Ace);

    assert!(card1.is_same_color(&card2));
    assert!(!card1.is_same_color(&card3));
}

#[test]
fn test_card_display() {
    let ace_of_spades = Card::new(Suit::Spades, Rank::Ace);
    assert_eq!(format!("{}", ace_of_spades), "A♠");

    let king_of_hearts = Card::new(Suit::Hearts, Rank::King);
    assert_eq!(format!("{}", king_of_hearts), "K♥");

    let two_of_clubs = Card::new(Suit::Clubs, Rank::Two);
    assert_eq!(format!("{}", two_of_clubs), "2♣");
}

#[test]
fn test_card_to_string() {
    let ace_of_spades = Card::new(Suit::Spades, Rank::Ace);
    assert_eq!(ace_of_spades.to_string(), "A♠");
}

#[test]
fn test_card_display_ascii() {
    let card = Card::new(Suit::Hearts, Rank::Ace);
    let expected = "┌─────┐\n│A   │\n│  ♥  │\n│   A│\n└─────┘";
    assert_eq!(card.display_ascii(), expected);
}

#[test]
fn test_display_ascii_structure() {
    let card = Card::new(Suit::Spades, Rank::King);
    let ascii = card.display_ascii();
    let lines: Vec<&str> = ascii.lines().collect();

    assert_eq!(lines.len(), 5);
    assert_eq!(lines[0], "┌─────┐");
    assert_eq!(lines[4], "└─────┘");
    assert!(lines[1].contains("K"));
    assert!(lines[2].contains("♠"));
    assert!(lines[3].contains("K"));
}

#[test]
fn test_card_serialization() {
    let card = Card::new(Suit::Hearts, Rank::Ace);
    let serialized = serde_json::to_string(&card).unwrap();
    let deserialized: Card = serde_json::from_str(&serialized).unwrap();
    assert_eq!(card, deserialized);
}

#[test]
fn test_card_try_from_u8() {
    let card = Card::try_from(0u8).unwrap();
    assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
}

#[test]
fn test_card_try_from_i8() {
    let card = Card::try_from(0i8).unwrap();
    assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
}

#[test]
fn test_card_try_from_u16() {
    let card = Card::try_from(0u16).unwrap();
    assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
}

#[test]
fn test_card_try_from_i16() {
    let card = Card::try_from(0i16).unwrap();
    assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
}

#[test]
fn test_card_try_from_u32() {
    let card = Card::try_from(0u32).unwrap();
    assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
}

#[test]
fn test_card_try_from_i32() {
    let card = Card::try_from(0i32).unwrap();
    assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
}

#[test]
fn test_card_try_from_u64() {
    let card = Card::try_from(0u64).unwrap();
    assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
}

#[test]
fn test_card_try_from_i64() {
    let card = Card::try_from(0i64).unwrap();
    assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
}

#[test]
fn test_card_try_from_usize() {
    let card = Card::try_from(0usize).unwrap();
    assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
}

#[test]
fn test_card_try_from_isize() {
    let card = Card::try_from(0isize).unwrap();
    assert_eq!(card, Card::new(Suit::Hearts, Rank::Two));
}

#[test]
fn test_card_try_from_u8_out_of_range() {
    let result = Card::try_from(56u8);
    assert!(result.is_err());
}

#[test]
fn test_card_try_from_i8_out_of_range() {
    let result = Card::try_from(56i8);
    assert!(result.is_err());
}

#[test]
fn test_card_try_from_u16_out_of_range() {
    let result = Card::try_from(56u16);
    assert!(result.is_err());
}

#[test]
fn test_card_try_from_i16_out_of_range() {
    let result = Card::try_from(56i16);
    assert!(result.is_err());
}

#[test]
fn test_card_try_from_u32_out_of_range() {
    let result = Card::try_from(56u32);
    assert!(result.is_err());
}

#[test]
fn test_card_try_from_i32_out_of_range() {
    let result = Card::try_from(56i32);
    assert!(result.is_err());
}

#[test]
fn test_card_try_from_u64_out_of_range() {
    let result = Card::try_from(56u64);
    assert!(result.is_err());
}

#[test]
fn test_card_try_from_i64_out_of_range() {
    let result = Card::try_from(56i64);
    assert!(result.is_err());
}

#[test]
fn test_card_try_from_usize_out_of_range() {
    let result = Card::try_from(56);
    assert!(result.is_err());
}

#[test]
fn test_card_try_from_isize_out_of_range() {
    let result = Card::try_from(56isize);
    assert!(result.is_err());
}

#[test]
fn test_card_u8_conversion() {
    let card = Card::new(Suit::Hearts, Rank::Two);
    assert_eq!(u8::from(card), 0);

    let card = Card::new(Suit::Hearts, Rank::Joker);
    assert_eq!(u8::from(card), 13);

    let card = Card::new(Suit::Diamonds, Rank::Two);
    assert_eq!(u8::from(card), 14);

    let card = Card::new(Suit::Spades, Rank::Joker);
    assert_eq!(u8::from(card), 55);
}

#[test]
fn test_card_i8_conversion() {
    let card = Card::new(Suit::Hearts, Rank::Two);
    assert_eq!(i8::from(card), 0);

    let card = Card::new(Suit::Hearts, Rank::Joker);
    assert_eq!(i8::from(card), 13);

    let card = Card::new(Suit::Diamonds, Rank::Two);
    assert_eq!(i8::from(card), 14);

    let card = Card::new(Suit::Spades, Rank::Joker);
    assert_eq!(i8::from(card), 55);
}

#[test]
fn test_card_u16_conversion() {
    let card = Card::new(Suit::Hearts, Rank::Two);
    assert_eq!(u16::from(card), 0);

    let card = Card::new(Suit::Hearts, Rank::Joker);
    assert_eq!(u16::from(card), 13);

    let card = Card::new(Suit::Diamonds, Rank::Two);
    assert_eq!(u16::from(card), 14);

    let card = Card::new(Suit::Spades, Rank::Joker);
    assert_eq!(u16::from(card), 55);
}

#[test]
fn test_card_i16_conversion() {
    let card = Card::new(Suit::Hearts, Rank::Two);
    assert_eq!(i16::from(card), 0);

    let card = Card::new(Suit::Hearts, Rank::Joker);
    assert_eq!(i16::from(card), 13);

    let card = Card::new(Suit::Diamonds, Rank::Two);
    assert_eq!(i16::from(card), 14);

    let card = Card::new(Suit::Spades, Rank::Joker);
    assert_eq!(i16::from(card), 55);
}

#[test]
fn test_card_u32_conversion() {
    let card = Card::new(Suit::Hearts, Rank::Two);
    assert_eq!(u32::from(card), 0);

    let card = Card::new(Suit::Hearts, Rank::Joker);
    assert_eq!(u32::from(card), 13);

    let card = Card::new(Suit::Diamonds, Rank::Two);
    assert_eq!(u32::from(card), 14);

    let card = Card::new(Suit::Spades, Rank::Joker);
    assert_eq!(u32::from(card), 55);
}

#[test]
fn test_card_i32_conversion() {
    let card = Card::new(Suit::Hearts, Rank::Two);
    assert_eq!(i32::from(card), 0);

    let card = Card::new(Suit::Hearts, Rank::Joker);
    assert_eq!(i32::from(card), 13);

    let card = Card::new(Suit::Diamonds, Rank::Two);
    assert_eq!(i32::from(card), 14);

    let card = Card::new(Suit::Spades, Rank::Joker);
    assert_eq!(i32::from(card), 55);
}

#[test]
fn test_card_u64_conversion() {
    let card = Card::new(Suit::Hearts, Rank::Two);
    assert_eq!(u64::from(card), 0);

    let card = Card::new(Suit::Hearts, Rank::Joker);
    assert_eq!(u64::from(card), 13);

    let card = Card::new(Suit::Diamonds, Rank::Two);
    assert_eq!(u64::from(card), 14);

    let card = Card::new(Suit::Spades, Rank::Joker);
    assert_eq!(u64::from(card), 55);
}

#[test]
fn test_card_i64_conversion() {
    let card = Card::new(Suit::Hearts, Rank::Two);
    assert_eq!(i64::from(card), 0);

    let card = Card::new(Suit::Hearts, Rank::Joker);
    assert_eq!(i64::from(card), 13);

    let card = Card::new(Suit::Diamonds, Rank::Two);
    assert_eq!(i64::from(card), 14);

    let card = Card::new(Suit::Spades, Rank::Joker);
    assert_eq!(i64::from(card), 55);
}

#[test]
fn test_card_usize_conversion() {
    let card = Card::new(Suit::Hearts, Rank::Two);
    assert_eq!(usize::from(card), 0);

    let card = Card::new(Suit::Hearts, Rank::Joker);
    assert_eq!(usize::from(card), 13);

    let card = Card::new(Suit::Diamonds, Rank::Two);
    assert_eq!(usize::from(card), 14);

    let card = Card::new(Suit::Spades, Rank::Joker);
    assert_eq!(usize::from(card), 55);
}

#[test]
fn test_card_isize_conversion() {
    let card = Card::new(Suit::Hearts, Rank::Two);
    assert_eq!(isize::from(card), 0);

    let card = Card::new(Suit::Hearts, Rank::Joker);
    assert_eq!(isize::from(card), 13);

    let card = Card::new(Suit::Diamonds, Rank::Two);
    assert_eq!(isize::from(card), 14);

    let card = Card::new(Suit::Spades, Rank::Joker);
    assert_eq!(isize::from(card), 55);
}

#[test]
fn test_card_hash_in_hashset() {
    let mut set: HashSet<Card> = HashSet::new();
    let ace_of_spades = Card::new(Suit::Spades, Rank::Ace);
    let king_of_hearts = Card::new(Suit::Hearts, Rank::King);

    assert!(set.insert(ace_of_spades));
    assert!(set.insert(king_of_hearts));
    assert!(!set.insert(ace_of_spades)); // Duplicate

    assert_eq!(set.len(), 2);
    assert!(set.contains(&ace_of_spades));
}

#[test]
fn test_card_hash_in_hashmap() {
    let mut map: HashMap<Card, u32> = HashMap::new();
    let ace_of_spades = Card::new(Suit::Spades, Rank::Ace);

    map.insert(ace_of_spades, 10);
    assert_eq!(map.get(&ace_of_spades), Some(&10));

    map.insert(ace_of_spades, 20);
    assert_eq!(map.get(&ace_of_spades), Some(&20));
    assert_eq!(map.len(), 1);
}

#[test]
fn test_card_hash_consistency() {
    let card1 = Card::new(Suit::Hearts, Rank::Ace);
    let card2 = Card::new(Suit::Hearts, Rank::Ace);

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    card1.hash(&mut hasher1);
    card2.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn test_different_cards_different_hashes() {
    let card1 = Card::new(Suit::Hearts, Rank::Ace);
    let card2 = Card::new(Suit::Spades, Rank::King);

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    card1.hash(&mut hasher1);
    card2.hash(&mut hasher2);

    assert_ne!(hasher1.finish(), hasher2.finish());
}
