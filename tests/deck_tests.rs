use crusty_cards::{AceLowComparator, Card, Deck, Rank, StandardComparator, Suit, TrumpComparator};
use std::collections::VecDeque;
use std::str::FromStr;

#[test]
fn test_deck_creation() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards.clone());
    assert_eq!(deck.len(), 2);
    assert_eq!(deck.peek(), Some(&cards[0]));
}

#[test]
fn test_deck_default() {
    let deck = Deck::default();
    assert!(deck.is_empty());
}

#[test]
fn test_deck_cut() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
        Card::new(Suit::Clubs, Rank::Jack),
    ]);
    let mut deck = Deck::new(cards);
    let result = deck.cut(2);
    assert_eq!(deck.peek(), Some(&Card::new(Suit::Diamonds, Rank::Queen)));
    assert!(result);

    let result = deck.cut(10); // Cutting beyond length should do nothing
    assert!(!result);
    assert_eq!(deck.peek(), Some(&Card::new(Suit::Diamonds, Rank::Queen)));

    let mut empty_deck = Deck::default();
    assert!(empty_deck.is_empty());
    assert_eq!(empty_deck.cut(0), false);
}

#[test]
fn test_deck_is_empty() {
    let cards = VecDeque::new();
    let mut deck = Deck::new(cards);
    assert!(deck.is_empty());
    deck.add_card(Card::new(Suit::Hearts, Rank::Ace));
    assert!(!deck.is_empty());
}

#[test]
fn test_deck_display() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);
    let display = format!("{}", deck);
    let card1 = Card::new(Suit::Hearts, Rank::Ace);
    let card2 = Card::new(Suit::Spades, Rank::King);
    assert_eq!(display, format!("{} {}", card1, card2));
}

#[test]
fn test_deck_shuffle() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
        Card::new(Suit::Clubs, Rank::Jack),
    ]);
    let mut deck = Deck::new(cards);
    let original_order = format!("{}", deck);
    // Try shuffling multiple times to ensure order changes (very low chance of false negative)
    for _ in 0..100 {
        deck.shuffle_times(1);
        let shuffled_order = format!("{}", deck);
        if original_order != shuffled_order {
            return;
        }
    }
    assert!(
        false,
        "Congratulations you just shuffled the deck randomly 100 times without changing its order, You should go buy a lottery ticket"
    );
}

#[test]
fn test_deck_shuffle_empty() {
    let mut empty_deck = Deck::default();
    assert!(empty_deck.is_empty());
    empty_deck.shuffle();
    assert!(empty_deck.is_empty());
}

#[test]
fn test_deck_deal() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);
    let dealt_card = deck.deal();
    assert_eq!(dealt_card, Some(Card::new(Suit::Hearts, Rank::Ace)));
    assert_eq!(deck.len(), 1);
    let dealt_card2 = deck.deal();
    assert_eq!(dealt_card2, Some(Card::new(Suit::Spades, Rank::King)));
    assert_eq!(deck.len(), 0);
    let dealt_card3 = deck.deal();
    assert_eq!(dealt_card3, None);
}

#[test]
fn test_deck_deal_bottom() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);
    let dealt_card = deck.deal_bottom();
    assert_eq!(dealt_card, Some(Card::new(Suit::Spades, Rank::King)));
    assert_eq!(deck.len(), 1);
    let dealt_card2 = deck.deal_bottom();
    assert_eq!(dealt_card2, Some(Card::new(Suit::Hearts, Rank::Ace)));
    assert_eq!(deck.len(), 0);
    let dealt_card3 = deck.deal_bottom();
    assert_eq!(dealt_card3, None);
}

#[test]
fn test_deck_add_card() {
    let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
    let mut deck = Deck::new(cards);
    deck.add_card(Card::new(Suit::Spades, Rank::King));
    assert_eq!(deck.len(), 2);
    assert_eq!(deck.peek(), Some(&Card::new(Suit::Spades, Rank::King)));
}

#[test]
fn test_deck_add_card_bottom() {
    let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
    let mut deck = Deck::new(cards);
    deck.add_card_bottom(Card::new(Suit::Spades, Rank::King));
    assert_eq!(deck.len(), 2);
    assert_eq!(
        deck.peek_bottom(),
        Some(&Card::new(Suit::Spades, Rank::King))
    );
}

#[test]
fn test_deck_peek() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);
    let top_card = deck.peek();
    assert_eq!(top_card, Some(&Card::new(Suit::Hearts, Rank::Ace)));
    assert_eq!(deck.len(), 2);
    deck.clear();
    let empty_peek = deck.peek();
    assert_eq!(empty_peek, None);
}

#[test]
fn test_deck_peek_bottom() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);
    let bottom_card = deck.peek_bottom();
    assert_eq!(bottom_card, Some(&Card::new(Suit::Spades, Rank::King)));
    assert_eq!(deck.len(), 2);
    deck.clear();
    let empty_peek = deck.peek_bottom();
    assert_eq!(empty_peek, None);
}

#[test]
fn test_deck_clear() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);
    deck.clear();
    assert_eq!(deck.len(), 0);
}

#[test]
fn test_deck_serialization() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);
    let serialized = serde_json::to_string(&deck).unwrap();
    let deserialized: Deck = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deck.len(), deserialized.len());
    assert_eq!(deck.to_string(), deserialized.to_string());
}

#[test]
fn test_deck_add_card_operator() {
    let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
    let deck = Deck::new(cards);
    let new_deck = deck + Card::new(Suit::Spades, Rank::King);
    assert_eq!(new_deck.len(), 2);
    assert_eq!(new_deck.peek(), Some(&Card::new(Suit::Spades, Rank::King)));
}

#[test]
fn test_deck_add_card_assign_operator() {
    let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
    let mut deck = Deck::new(cards);
    deck += Card::new(Suit::Spades, Rank::King);
    assert_eq!(deck.len(), 2);
    assert_eq!(deck.peek(), Some(&Card::new(Suit::Spades, Rank::King)));
}

#[test]
fn test_deck_sub_card() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let deck = Deck::new(cards);
    let subtracted_deck = deck - Card::new(Suit::Hearts, Rank::Ace);
    assert_eq!(subtracted_deck.len(), 2);
    assert_eq!(
        subtracted_deck.peek(),
        Some(&Card::new(Suit::Spades, Rank::King))
    );
}

#[test]
fn test_deck_sub_assign_card() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let mut deck = Deck::new(cards);
    deck -= Card::new(Suit::Hearts, Rank::Ace);
    assert_eq!(deck.len(), 2);
    assert_eq!(deck.peek(), Some(&Card::new(Suit::Spades, Rank::King)));
}

#[test]
fn test_deck_sub_missing_card() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);
    deck -= Card::new(Suit::Diamonds, Rank::Queen);
    assert_eq!(deck.len(), 2);
    assert_eq!(deck.peek(), Some(&Card::new(Suit::Hearts, Rank::Ace)));
}

#[test]
fn test_add_deck_deck() {
    let cards1 = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let cards2 = VecDeque::from(vec![
        Card::new(Suit::Diamonds, Rank::Queen),
        Card::new(Suit::Clubs, Rank::Jack),
    ]);
    let deck1 = Deck::new(cards1);
    let deck2 = Deck::new(cards2);
    let combined_deck = deck1 + deck2;
    assert_eq!(combined_deck.len(), 4);
}

#[test]
fn test_add_assign_deck_deck() {
    let cards1 = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let cards2 = VecDeque::from(vec![
        Card::new(Suit::Diamonds, Rank::Queen),
        Card::new(Suit::Clubs, Rank::Jack),
    ]);
    let mut deck1 = Deck::new(cards1);
    let deck2 = Deck::new(cards2);
    deck1 += deck2;
    assert_eq!(deck1.len(), 4);
}

#[test]
fn test_deck_sub_deck() {
    let cards1 = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let cards2 = VecDeque::from(vec![
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Clubs, Rank::Jack),
    ]);
    let deck1 = Deck::new(cards1);
    let deck2 = Deck::new(cards2);
    let subtracted_deck = deck1 - deck2;
    assert_eq!(subtracted_deck.len(), 2);
    assert_eq!(
        subtracted_deck.peek(),
        Some(&Card::new(Suit::Hearts, Rank::Ace))
    );
}

#[test]
fn test_deck_sub_assign_deck() {
    let cards1 = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let cards2 = VecDeque::from(vec![
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Clubs, Rank::Jack),
    ]);
    let mut deck1 = Deck::new(cards1);
    let deck2 = Deck::new(cards2);
    deck1 -= deck2;
    assert_eq!(deck1.len(), 2);
    assert_eq!(deck1.peek(), Some(&Card::new(Suit::Hearts, Rank::Ace)));
}

#[test]
fn test_deck_mul() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);
    let multiplied_deck = deck * 3;
    assert_eq!(multiplied_deck.len(), 6);
}

#[test]
fn test_deck_mul_zero() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);
    let multiplied_deck = deck * 0;
    assert_eq!(multiplied_deck.len(), 0);
}

#[test]
fn test_deck_mul_one() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);
    let multiplied_deck = deck * 1;
    assert_eq!(multiplied_deck.len(), 2);
}

#[test]
fn test_deck_mul_assign() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);
    deck *= 3;
    assert_eq!(deck.len(), 6);
}

#[test]
fn test_deck_try_from_vec_usize() {
    let values = vec![0, 12, 25];
    let deck = Deck::try_from(values).unwrap();
    assert_eq!(deck.len(), 3);
}

#[test]
fn test_deck_from_deck_to_vec_usize() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);
    let values: Vec<usize> = Vec::from(deck);
    assert_eq!(values, vec![12, 53]);
}

// === Sorting tests ===

#[test]
fn test_deck_sort_by_comparator() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::King),
        Card::new(Suit::Spades, Rank::Two),
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Seven),
    ]);
    let mut deck = Deck::new(cards);

    deck.sort_by_comparator(&StandardComparator);

    // Should be sorted by rank: 2, 7, K, A
    assert_eq!(deck.deal().unwrap().rank(), Rank::Two);
    assert_eq!(deck.deal().unwrap().rank(), Rank::Seven);
    assert_eq!(deck.deal().unwrap().rank(), Rank::King);
    assert_eq!(deck.deal().unwrap().rank(), Rank::Ace);
}

#[test]
fn test_deck_sort_by_ace_low_comparator() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::King),
        Card::new(Suit::Spades, Rank::Two),
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Seven),
    ]);
    let mut deck = Deck::new(cards);

    deck.sort_by_comparator(&AceLowComparator);

    // Should be sorted with Ace low: A, 2, 7, K
    assert_eq!(deck.deal().unwrap().rank(), Rank::Ace);
    assert_eq!(deck.deal().unwrap().rank(), Rank::Two);
    assert_eq!(deck.deal().unwrap().rank(), Rank::Seven);
    assert_eq!(deck.deal().unwrap().rank(), Rank::King);
}

#[test]
fn test_deck_sort_by_custom_function() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::King),
        Card::new(Suit::Spades, Rank::Two),
        Card::new(Suit::Clubs, Rank::Ace),
    ]);
    let mut deck = Deck::new(cards);

    // Sort descending by rank value
    deck.sort_by(|a, b| b.rank().value().cmp(&a.rank().value()));

    // Should be sorted descending: A, K, 2
    assert_eq!(deck.deal().unwrap().rank(), Rank::Ace);
    assert_eq!(deck.deal().unwrap().rank(), Rank::King);
    assert_eq!(deck.deal().unwrap().rank(), Rank::Two);
}

#[test]
fn test_deck_sort_by_trump_comparator() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Spades, Rank::Ace),   // Non-trump Ace
        Card::new(Suit::Hearts, Rank::Two),   // Trump Two (hearts is trump)
        Card::new(Suit::Clubs, Rank::King),   // Non-trump King
        Card::new(Suit::Hearts, Rank::Seven), // Trump Seven
    ]);
    let mut deck = Deck::new(cards);

    deck.sort_by_comparator(&TrumpComparator::new(Suit::Hearts));

    // Non-trump cards first (by rank), then trump cards (by rank)
    // King < Ace (non-trump), then Two < Seven (trump)
    let card1 = deck.deal().unwrap();
    let card2 = deck.deal().unwrap();
    let card3 = deck.deal().unwrap();
    let card4 = deck.deal().unwrap();

    // Non-trump: King, Ace
    assert_eq!(card1.suit(), Suit::Clubs);
    assert_eq!(card1.rank(), Rank::King);
    assert_eq!(card2.suit(), Suit::Spades);
    assert_eq!(card2.rank(), Rank::Ace);

    // Trump: Two, Seven
    assert_eq!(card3.suit(), Suit::Hearts);
    assert_eq!(card3.rank(), Rank::Two);
    assert_eq!(card4.suit(), Suit::Hearts);
    assert_eq!(card4.rank(), Rank::Seven);
}

#[test]
fn test_deck_iter() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);

    let mut iter = deck.iter();
    assert_eq!(iter.next().unwrap().rank(), Rank::Ace);
    assert_eq!(iter.next().unwrap().rank(), Rank::King);
    assert!(iter.next().is_none());
}

#[test]
fn test_deck_into_iter() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);

    let mut iter = deck.into_iter();
    assert_eq!(iter.next().unwrap().rank(), Rank::Ace);
    assert_eq!(iter.next().unwrap().rank(), Rank::King);
    assert!(iter.next().is_none());
}

#[test]
fn test_deck_iter_mut() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);

    for card in deck.iter_mut() {
        if card.rank() == Rank::Ace {
            *card = Card::new(Suit::Hearts, Rank::Two); // Change Ace to Two
        }
    }

    let mut iter = deck.iter();
    assert_eq!(iter.next().unwrap().rank(), Rank::Two);
    assert_eq!(iter.next().unwrap().rank(), Rank::King);
}

#[test]
fn test_deck_into_iter_mut() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);

    for card in &mut deck {
        if card.rank() == Rank::Ace {
            *card = Card::new(Suit::Hearts, Rank::Two);
        }
    }
}

#[test]
fn test_deck_from_str_delimiter() {
    let deck_str = "2d,3hearts,4s,5c,aCeSpades";
    let mut deck = Deck::from_str_delimiter(deck_str, ',').unwrap();

    assert_eq!(deck.len(), 5);
    assert_eq!(deck.deal().unwrap(), Card::new(Suit::Diamonds, Rank::Two));
    assert_eq!(deck.deal().unwrap(), Card::new(Suit::Hearts, Rank::Three));
    assert_eq!(deck.deal().unwrap(), Card::new(Suit::Spades, Rank::Four));
    assert_eq!(deck.deal().unwrap(), Card::new(Suit::Clubs, Rank::Five));
    assert_eq!(deck.deal().unwrap(), Card::new(Suit::Spades, Rank::Ace));

    let deck_str_invalid = "2d,invalid_card,4s";
    let result = Deck::from_str_delimiter(deck_str_invalid, ',');
    assert!(result.is_err());
}

#[test]
fn test_to_and_from_str() {
    let mut deck = Deck::new(VecDeque::from(vec![
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Hearts, Rank::Three),
        Card::new(Suit::Spades, Rank::Four),
        Card::new(Suit::Clubs, Rank::Five),
        Card::new(Suit::Spades, Rank::Ace),
    ]));

    let deck_str = format!("{}", deck);
    let mut new_deck = Deck::from_str(&deck_str).unwrap();

    assert_eq!(deck.len(), new_deck.len());
    while let (Some(card1), Some(card2)) = (deck.deal(), new_deck.deal()) {
        assert_eq!(card1, card2);
    }
}

#[test]
fn test_deck_to_and_from_json() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let deck = Deck::new(cards);

    // Convert to JSON and back
    let json = deck.to_json().unwrap();
    let restored = Deck::from_json(&json).unwrap();

    assert_eq!(deck.len(), restored.len());

    // Verify cards match
    for (original, restored) in deck.iter().zip(restored.iter()) {
        assert_eq!(original, restored);
    }
}

#[test]
fn test_deck_to_json_pretty() {
    let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
    let deck = Deck::new(cards);

    let json_pretty = deck.to_json_pretty().unwrap();

    // Pretty JSON should contain newlines
    assert!(json_pretty.contains('\n'));

    // Should still be valid JSON
    let restored = Deck::from_json(&json_pretty).unwrap();
    assert_eq!(deck.len(), restored.len());
}

#[test]
fn test_deck_to_and_from_yaml() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let deck = Deck::new(cards);

    // Convert to YAML and back
    let yaml = deck.to_yaml().unwrap();
    let restored = Deck::from_yaml(&yaml).unwrap();

    assert_eq!(deck.len(), restored.len());

    // Verify cards match
    for (original, restored) in deck.iter().zip(restored.iter()) {
        assert_eq!(original, restored);
    }
}

#[test]
fn test_deck_json_format() {
    let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
    let deck = Deck::new(cards);

    let json = deck.to_json().unwrap();

    // Verify it's valid JSON structure
    assert!(json.contains("cards"));
    assert!(json.contains("suit"));
    assert!(json.contains("rank"));
}

#[test]
fn test_deck_yaml_format() {
    let cards = VecDeque::from(vec![Card::new(Suit::Hearts, Rank::Ace)]);
    let deck = Deck::new(cards);

    let yaml = deck.to_yaml().unwrap();

    // YAML should contain readable structure
    assert!(yaml.contains("cards"));
    assert!(yaml.contains("suit"));
    assert!(yaml.contains("rank"));
}

#[test]
fn test_deck_empty_json_roundtrip() {
    let deck = Deck::new(VecDeque::new());

    let json = deck.to_json().unwrap();
    let restored = Deck::from_json(&json).unwrap();

    assert!(restored.is_empty());
}

#[test]
fn test_deck_empty_yaml_roundtrip() {
    let deck = Deck::new(VecDeque::new());

    let yaml = deck.to_yaml().unwrap();
    let restored = Deck::from_yaml(&yaml).unwrap();

    assert!(restored.is_empty());
}

#[test]
fn test_deck_to_and_from_csv() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let deck = Deck::new(cards);

    let csv = deck.as_csv();
    let expected = "Rank,Suit\nA,♥\nK,♠\nQ,♦\n";
    assert_eq!(csv, expected);

    let restored = Deck::from_csv(&csv).unwrap();
    assert_eq!(deck.len(), restored.len());

    // Verify cards match
    for (original, restored) in deck.iter().zip(restored.iter()) {
        assert_eq!(original, restored);
    }
}

#[test]
fn test_deck_equality() {
    let cards1 = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck1 = Deck::new(cards1);

    let cards2 = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck2 = Deck::new(cards2);

    assert_eq!(deck1, deck2);
}

#[test]
fn test_deck_inequality() {
    let cards1 = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck1 = Deck::new(cards1);

    let cards2 = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::Queen),
    ]);
    let deck2 = Deck::new(cards2);

    assert_ne!(deck1, deck2);
}

#[test]
fn test_deck_contains() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);

    assert!(deck.contains(&Card::new(Suit::Hearts, Rank::Ace)));
    assert!(!deck.contains(&Card::new(Suit::Diamonds, Rank::Queen)));
}

#[test]
fn test_deck_find() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);

    let found_position = deck.find(&Card::new(Suit::Spades, Rank::King));
    assert_eq!(found_position, Some(1));

    let found_position_none = deck.find(&Card::new(Suit::Diamonds, Rank::Queen));
    assert_eq!(found_position_none, None);
}

#[test]
fn test_deck_insert_at() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);

    let new_card = Card::new(Suit::Diamonds, Rank::Queen);
    assert!(deck.insert_at(new_card, 1));

    // Verify the card was inserted
    let found_position = deck.find(&Card::new(Suit::Diamonds, Rank::Queen));
    assert_eq!(found_position, Some(1));

    let new_card = Card::new(Suit::Clubs, Rank::Ten);
    assert!(!deck.insert_at(new_card, 1000));

    // Verify the card was not inserted
    let found_position = deck.find(&Card::new(Suit::Clubs, Rank::Ten));
    assert_eq!(found_position, None);

    // Verify we can add to end of deck
    let new_card = Card::new(Suit::Clubs, Rank::Ten);
    assert!(deck.insert_at(new_card, deck.len()));
    assert_eq!(
        deck.find(&Card::new(Suit::Clubs, Rank::Ten)),
        Some(deck.len() - 1)
    );

    // Verify we can add to the beginning of a deck
    let new_card = Card::new(Suit::Clubs, Rank::Nine);
    assert!(deck.insert_at(new_card, 0));
    assert_eq!(deck.find(&Card::new(Suit::Clubs, Rank::Nine)), Some(0));
}

#[test]
fn test_deck_remove_at() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let mut deck = Deck::new(cards);

    assert_eq!(deck.remove_at(1), Some(Card::new(Suit::Spades, Rank::King)));
    assert_eq!(deck.len(), 2);
    assert_eq!(deck.remove_at(100), None);
}

#[test]
fn test_deck_count() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);

    let count_ace_hearts = deck.count(&Card::new(Suit::Hearts, Rank::Ace));
    assert_eq!(count_ace_hearts, 2);

    let count_king_spades = deck.count(&Card::new(Suit::Spades, Rank::King));
    assert_eq!(count_king_spades, 1);

    let count_queen_diamonds = deck.count(&Card::new(Suit::Diamonds, Rank::Queen));
    assert_eq!(count_queen_diamonds, 0);
}

#[test]
fn test_deck_reverse() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Clubs, Rank::Ten),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let mut deck = Deck::new(cards);

    deck.reverse();

    assert_eq!(deck.deal().unwrap(), Card::new(Suit::Diamonds, Rank::Queen));
    assert_eq!(deck.deal().unwrap(), Card::new(Suit::Clubs, Rank::Ten));
    assert_eq!(deck.deal().unwrap(), Card::new(Suit::Spades, Rank::King));
    assert_eq!(deck.deal().unwrap(), Card::new(Suit::Hearts, Rank::Ace));
}

#[test]
fn test_deck_split_at() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let deck = Deck::new(cards);

    let (left, right) = deck.split_at(2);
    assert_eq!(left.len(), 2);
    assert_eq!(right.len(), 1);
    assert_eq!(left.peek(), Some(&Card::new(Suit::Hearts, Rank::Ace)));
    assert_eq!(right.peek(), Some(&Card::new(Suit::Diamonds, Rank::Queen)));

    // Split at 0
    let (left, right) = deck.split_at(0);
    assert_eq!(left.len(), 0);
    assert_eq!(right.len(), 3);
    assert_eq!(right.peek(), Some(&Card::new(Suit::Hearts, Rank::Ace)));

    // Split at Len
    let (left, right) = deck.split_at(deck.len());
    assert_eq!(left.len(), 3);
    assert_eq!(right.len(), 0);
    assert_eq!(left.peek(), Some(&Card::new(Suit::Hearts, Rank::Ace)));

    //Split at len + 1
    let (left, right) = deck.split_at(deck.len() + 1);
    assert_eq!(left.len(), 3);
    assert_eq!(right.len(), 0);
    assert_eq!(left.peek(), Some(&Card::new(Suit::Hearts, Rank::Ace)));
}

#[test]
fn test_deck_peek_at() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let deck = Deck::new(cards);

    assert_eq!(deck.peek_at(0), Some(&Card::new(Suit::Hearts, Rank::Ace)));
    assert_eq!(deck.peek_at(1), Some(&Card::new(Suit::Spades, Rank::King)));
    assert_eq!(
        deck.peek_at(2),
        Some(&Card::new(Suit::Diamonds, Rank::Queen))
    );
    assert_eq!(deck.peek_at(3), None);
}

#[test]
fn test_deck_deal_n() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
        Card::new(Suit::Clubs, Rank::Jack),
    ]);
    let mut deck = Deck::new(cards);

    let dealt_cards = deck.deal_n(2).unwrap();
    assert_eq!(dealt_cards.len(), 2);
    assert_eq!(dealt_cards[0], Card::new(Suit::Hearts, Rank::Ace));
    assert_eq!(dealt_cards[1], Card::new(Suit::Spades, Rank::King));
    assert_eq!(deck.len(), 2);

    let dealt_cards_empty = deck.deal_n(0).unwrap();
    assert_eq!(dealt_cards_empty.len(), 0);

    let dealt_cards_exceed = deck.deal_n(deck.len() + 1);
    assert_eq!(dealt_cards_exceed, None);
}

#[test]
fn test_deck_deal_n_bottom() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
        Card::new(Suit::Clubs, Rank::Jack),
    ]);
    let mut deck = Deck::new(cards);

    let dealt_cards = deck.deal_n(2).unwrap();
    assert_eq!(dealt_cards.len(), 2);
    assert_eq!(dealt_cards[0], Card::new(Suit::Hearts, Rank::Ace));
    assert_eq!(dealt_cards[1], Card::new(Suit::Spades, Rank::King));
    assert_eq!(deck.len(), 2);

    let dealt_cards_empty = deck.deal_n(0).unwrap();
    assert_eq!(dealt_cards_empty.len(), 0);

    let dealt_cards_exceed = deck.deal_n(deck.len() + 1);
    assert_eq!(dealt_cards_exceed, None);
}

#[test]
fn test_deck_clone() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let deck = Deck::new(cards);
    let mut cloned_deck = deck.clone();
    assert_eq!(deck.len(), cloned_deck.len());
    assert_eq!(deck.peek(), cloned_deck.peek());
    assert_eq!(deck, cloned_deck);
    cloned_deck.deal();
    assert_ne!(deck, cloned_deck);
}

#[test]
fn test_deck_json_roundtrip() {
    let mut cards = VecDeque::new();
    for suit in Suit::ALL {
        for rank in Rank::ALL {
            cards.push_back(Card::new(suit, rank));
        }
    }
    let deck = Deck::new(cards);
    let json = serde_json::to_string(&deck).unwrap();
    let deserialized_deck: Deck = serde_json::from_str(&json).unwrap();
    assert_eq!(deck, deserialized_deck);
}

#[test]
fn test_deck_yaml_roundtrip() {
    let mut cards = VecDeque::new();
    for suit in Suit::ALL {
        for rank in Rank::ALL {
            cards.push_back(Card::new(suit, rank));
        }
    }
    let deck = Deck::new(cards);
    let yaml = serde_yaml::to_string(&deck).unwrap();
    let deserialized_deck: Deck = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(deck, deserialized_deck);
}

#[test]
fn test_deck_csv_roundtrip() {
    let mut cards = VecDeque::new();
    for suit in Suit::ALL {
        for rank in Rank::ALL {
            cards.push_back(Card::new(suit, rank));
        }
    }
    let deck = Deck::new(cards);
    let csv = deck.as_csv();
    let deserialized_deck: Deck = Deck::from_csv(&csv).unwrap();
    assert_eq!(deck, deserialized_deck);
}

#[test]
fn test_deck_riffle_shuffle() {
    let mut cards = VecDeque::new();
    for suit in Suit::ALL {
        for rank in Rank::ALL {
            cards.push_back(Card::new(suit, rank));
        }
    }
    let mut deck = Deck::new(cards);
    let original_deck = deck.clone();
    deck.riffle_shuffle();
    assert_eq!(deck.len(), original_deck.len());
    assert_ne!(deck, original_deck);
}

#[test]
fn test_deck_riffle_shuffle_times() {
    let mut cards = VecDeque::new();
    for suit in Suit::ALL {
        for rank in Rank::ALL {
            cards.push_back(Card::new(suit, rank));
        }
    }
    let mut deck = Deck::new(cards);
    let original_deck = deck.clone();
    deck.riffle_shuffle_times(5);
    assert_eq!(deck.len(), original_deck.len());
    assert_ne!(deck, original_deck);
}

#[test]
fn test_deck_riffle_shuffle_two() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);
    let original_deck = deck.clone();
    deck.riffle_shuffle();
    assert_eq!(deck.len(), original_deck.len());
    assert_ne!(deck, original_deck); // There's a chance this could fail if shuffle results in same order
}

#[test]
fn test_deck_overhand_shuffle() {
    let mut cards = VecDeque::new();
    for suit in Suit::ALL {
        for rank in Rank::ALL {
            cards.push_back(Card::new(suit, rank));
        }
    }
    let mut deck = Deck::new(cards);
    let original_deck = deck.clone();
    deck.overhand_shuffle();
    assert_eq!(deck.len(), original_deck.len());
    assert_ne!(deck, original_deck);
}

#[test]
fn test_deck_overhand_shuffle_two() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);
    let mut deck = Deck::new(cards);
    let original_deck = deck.clone();
    deck.overhand_shuffle();
    assert_eq!(deck.len(), original_deck.len());
    assert_ne!(deck, original_deck); // There's a chance this could fail if shuffle results in same order
}

#[test]
fn test_deck_overhand_shuffle_times() {
    let mut cards = VecDeque::new();
    for suit in Suit::ALL {
        for rank in Rank::ALL {
            cards.push_back(Card::new(suit, rank));
        }
    }
    let mut deck = Deck::new(cards);
    let original_deck = deck.clone();
    for _ in 1..100 {
        deck.overhand_shuffle_times(10);
        if deck != original_deck {
            return;
        }
    }
    assert!(false, "Congratulations you just shuffled the deck randomly 100 times without changing its order, You should go buy a lottery ticket")
}

#[test]
fn test_deck_index() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let deck = Deck::new(cards);

    assert_eq!(deck[0], Card::new(Suit::Hearts, Rank::Ace));
    assert_eq!(deck[1], Card::new(Suit::Spades, Rank::King));
    assert_eq!(deck[2], Card::new(Suit::Diamonds, Rank::Queen));
}

#[test]
#[should_panic]
fn test_deck_index_panic() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let deck = Deck::new(cards);
    let _ = deck[3];
}

#[test]
fn test_deck_index_mut() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let mut deck = Deck::new(cards);
    deck[0] = Card::new(Suit::Clubs, Rank::Jack);
    assert_eq!(deck[0], Card::new(Suit::Clubs, Rank::Jack));
}

#[test]
#[should_panic]
fn test_deck_index_mut_panic() {
    let cards = VecDeque::from(vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let mut deck = Deck::new(cards);
    deck[3] = Card::new(Suit::Clubs, Rank::Jack);
}

#[test]
fn test_from_iterator() {
    let cards = vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
    ];
    let deck: Deck = cards.into_iter().collect();
    assert_eq!(deck.len(), 3);
}
