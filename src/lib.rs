//! # crusty_cards
//!
//! A Rust library for working with playing cards and decks.
//!
//! This crate provides flexible primitives for building card games, including:
//! - [`Card`], [`Suit`], [`Rank`], and [`Color`] types
//! - A [`Deck`] collection with shuffling, dealing, and manipulation methods
//! - Customizable card ordering via the [`CardComparator`] trait
//! - Deck generation via the [`DeckFactory`] trait
//! - Serialization support (JSON, YAML, CSV)
//!
//! ## Quick Start
//!
//! ```rust
//! use crusty_cards::{Deck, Standard52, Card, Suit, Rank};
//!
//! // Create a standard 52-card deck
//! let mut deck = Deck::from_factory(Standard52);
//!
//! // Shuffle and deal
//! deck.shuffle();
//! let card = deck.deal().unwrap();
//!
//! // Create specific cards
//! let ace_of_spades = Card::new(Suit::Spades, Rank::Ace);
//! ```
//!
//! ## Custom Deck Factories
//!
//! ```rust
//! use crusty_cards::{Card, Deck, DeckFactory, Suit, Rank};
//! use std::collections::VecDeque;
//!
//! struct PinochleDeck;
//!
//! impl DeckFactory for PinochleDeck {
//!     fn generate(&self) -> VecDeque<Card> {
//!         let ranks = [Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
//!         let mut cards = VecDeque::new();
//!         for _ in 0..2 {
//!             for suit in Suit::ALL {
//!                 for rank in ranks {
//!                     cards.push_back(Card::new(suit, rank));
//!                 }
//!             }
//!         }
//!         cards
//!     }
//! }
//!
//! let deck = Deck::from_factory(PinochleDeck);
//! assert_eq!(deck.len(), 48);
//! ```
//!
//! ## Thread Safety
//!
//! Both [`Card`] and [`Deck`] implement `Send` and `Sync`, making them safe
//! to use across threads. For concurrent mutable access, wrap in
//! `Arc<Mutex<Deck>>` or `Arc<RwLock<Deck>>`.

pub mod objects;

pub use objects::card::Card;
pub use objects::color::Color;
pub use objects::deck::Deck;
pub use objects::rank::Rank;
pub use objects::suit::Suit;

pub mod traits;

pub use traits::comparator::AceLowComparator;
pub use traits::comparator::BridgeComparator;
pub use traits::comparator::CardComparator;
pub use traits::comparator::StandardComparator;
pub use traits::comparator::TrumpComparator;
pub use traits::factory::DeckFactory;

pub mod utils;

pub use utils::standard::Standard52;
pub use utils::standard::Standard54;
