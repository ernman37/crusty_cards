use crate::Card;
use std::collections::VecDeque;

/// A trait for generating the cards in a [`Deck`](crate::Deck).
///
/// Implement this trait to create custom deck configurations for different games.
///
/// # Built-in Factories
///
/// - [`Standard52`](crate::Standard52) - Standard 52-card deck
/// - [`Standard54`](crate::Standard54) - 54-card deck with 2 jokers
///
/// # Examples
///
/// ```rust
/// use crusty_cards::{Card, Deck, DeckFactory, Suit, Rank};
/// use std::collections::VecDeque;
///
/// /// A 48-card Pinochle deck (9-A, two of each)
/// struct PinochleDeck;
///
/// impl DeckFactory for PinochleDeck {
///     fn generate(&self) -> VecDeque<Card> {
///         let ranks = [Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
///         let mut cards = VecDeque::new();
///         for _ in 0..2 {  // Two copies of each card
///             for suit in Suit::ALL {
///                 for rank in ranks {
///                     cards.push_back(Card::new(suit, rank));
///                 }
///             }
///         }
///         cards
///     }
/// }
///
/// let deck = Deck::from_factory(PinochleDeck);
/// assert_eq!(deck.len(), 48);
/// ```
pub trait DeckFactory {
    /// Generates the cards for a deck.
    ///
    /// Returns a `VecDeque<Card>` containing all cards in the desired order.
    fn generate(&self) -> VecDeque<Card>;
}
