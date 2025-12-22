pub mod objects;

pub use objects::card::Card;
pub use objects::card::Color;
pub use objects::card::Rank;
pub use objects::card::Suit;
pub use objects::deck::Deck;

pub mod traits;

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
