use crusty_cards::{
    AceLowComparator, Card, Deck, Rank, StandardComparator, Suit, TrumpComparator,
};
use std::collections::VecDeque;

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
    assert_eq!(display, format!("{} {} ", card1, card2));
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
        "Deck shuffle did not change order after multiple attempts"
    );
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
        Card::new(Suit::Spades, Rank::Ace),    // Non-trump Ace
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
