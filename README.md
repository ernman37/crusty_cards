# crusty_cards

[![Crates.io](https://img.shields.io/crates/v/crusty_cards.svg)](https://crates.io/crates/crusty_cards)
[![Documentation](https://docs.rs/crusty_cards/badge.svg)](https://docs.rs/crusty_cards)
[![CI](https://github.com/ernman37/crusty_cards/actions/workflows/rust_main.yml/badge.svg)](https://github.com/ernman37/crusty_cards/actions)
[![codecov](https://codecov.io/gh/ernman37/crusty_cards/branch/main/graph/badge.svg)](https://codecov.io/gh/ernman37/crusty_cards)

A Rust library for working with playing cards and decks. Provides flexible primitives for building card games.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
crusty_cards = "x.x.x"
```

## Quick Start

```rust
use crusty_cards::{Deck, Standard52, Card, Suit, Rank};

// Create a standard 52-card deck
let mut deck = Deck::from_factory(Standard52);

// Shuffle the deck
deck.shuffle();

// Deal cards
let card = deck.deal().unwrap();
println!("{}", card);  // e.g., "A♠"

// Create specific cards
let ace_of_spades = Card::new(Suit::Spades, Rank::Ace);
```

## Features

### Cards

```rust
use crusty_cards::{Card, Suit, Rank};

let card = Card::new(Suit::Hearts, Rank::Queen);

// Access properties
card.suit();   // Suit::Hearts
card.rank();   // Rank::Queen
card.color();  // Color::Red

// Parse from string (supports Unicode)
let card: Card = "K♠".parse().unwrap();
let card: Card = "King♠".parse().unwrap();
let card: Card = "KingS".parse().unwrap();
let card: Card = "Kspades".parse().unwrap();
let card: Card = "Kingspades".parse().unwrap();
let card: Card = "KiNgsPaDeS".parse().unwrap();
```

### Decks

```rust
use crusty_cards::{Deck, Standard52, Standard54};

// Create decks
let deck = Deck::from_factory(Standard52);      // 52 cards
let deck = Deck::from_factory(Standard54);      // 54 cards (with jokers)
let deck: Deck = "A♠ K♠ Q♠".parse().unwrap();   // From string

// Shuffling
deck.shuffle();              // Random shuffle
deck.riffle_shuffle();       // Riffle shuffle
deck.overhand_shuffle();     // Overhand shuffle
deck.cut(26);                // Cut at position

// Dealing
let card = deck.deal();              // From top
let card = deck.deal_bottom();       // From bottom
let cards = deck.deal_n(5);          // Multiple cards from top
let cards = deck.deal_n_bottom(5);   // Multiple cards from bottom

// Adding cards
deck.add_card(card);         // To top
deck.add_cards(cards);       // To top
deck.add_card_bottom(card);  // To bottom
deck.add_cards_bottom(card); // To bottom

// Inspection
deck.len();
deck.is_empty();
deck.peek();
deck.contains(&card);
deck.find(&card);
```

### Custom Sorting

```rust
use crusty_cards::{Deck, Standard52, StandardComparator, AceLowComparator};

let mut deck = Deck::from_factory(Standard52);

// Sort with Ace high (default)
deck.sort_by_comparator(&StandardComparator);

// Sort with Ace low
deck.sort_by_comparator(&AceLowComparator);

// Custom sort
deck.sort_by(|a, b| b.rank().value().cmp(&a.rank().value()));
```

### Serialization

```rust
use crusty_cards::{Deck, Standard52};

let deck = Deck::from_factory(Standard52);

// JSON
let json = deck.to_json().unwrap();
let deck = Deck::from_json(&json).unwrap();

// YAML
let yaml = deck.to_yaml().unwrap();
let deck = Deck::from_yaml(&yaml).unwrap();

// CSV
let csv = deck.as_csv();
let deck = Deck::from_csv(&csv).unwrap();

// Custom String
let custom = deck.as_str_delimiter('\n')
let deck = Deck::from_str_delimiter('\n')
```

### Operator Overloads

```rust
use crusty_cards::{Deck, Card, Suit, Rank};

let mut deck = Deck::default();
let card = Card::new(Suit::Spades, Rank::Ace);

// Add/remove cards
deck += card;           // Add card
deck -= card;           // Remove card

// Combine decks
let combined = deck1 + deck2;

// Duplicate deck
let double_deck = deck * 2;

// Index access
let card = deck[0];
deck[0] = another_card;
```

### Custom Deck Factories

```rust
use crusty_cards::{Card, Deck, DeckFactory, Suit, Rank};
use std::collections::VecDeque;

struct PinochleDeck;

impl DeckFactory for PinochleDeck {
    fn generate(&self) -> VecDeque<Card> {
        let ranks = [Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
        let mut cards = VecDeque::new();

        // Two of each card
        for _ in 0..2 {
            for suit in Suit::ALL {
                for rank in ranks {
                    cards.push_back(Card::new(suit, rank));
                }
            }
        }
        cards
    }
}

let deck = Deck::from_factory(PinochleDeck);
```

### Custom Card Comparators

```rust
use crusty_cards::{CardComparator, Card, Rank, Suit};
use std::cmp::Ordering;

struct EuchreComparator {
    trump: Suit,
}

impl CardComparator for EuchreComparator {
    fn rank_value(&self, rank: Rank) -> i32 {
        match rank {
            Rank::Nine => 0,
            Rank::Ten => 1,
            Rank::Jack => 2,  // Adjusted by suit in compare()
            Rank::Queen => 3,
            Rank::King => 4,
            Rank::Ace => 5,
            _ => -1,
        }
    }
}
```

## Thread Safety

`Card` and `Deck` are both `Send` and `Sync`. For concurrent mutable access, wrap in `Arc<Mutex<Deck>>` or `Arc<RwLock<Deck>>`.

## License

[BEERWARE](LICENSE) - If we meet one day and you think this stuff was worth it, you can buy me a beer in return.
