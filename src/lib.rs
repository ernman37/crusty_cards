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

#[cfg(test)]
mod tests {
    use super::{Deck, Standard52, Standard54};

    #[test]
    fn test_deck_from_standard_52() {
        let deck = Deck::from_factory(Standard52);
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn test_deck_from_standard_54() {
        let deck = Deck::from_factory(Standard54);
        assert_eq!(deck.len(), 54);
    }
}
