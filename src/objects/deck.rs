mod card;
pub use card::{Card, Color, Suit, Rank};

use std::collections::VecDeque;
use std::ops::{Add, Sub};

pub struct Deck {
    cards: VecDeque<Card>,
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
impl Deck {
    pub fn new(cards: VecDeque<Card>) -> Self {
        Deck { cards }
    }

    pub fn cut(&mut self, index: usize) {
        if index >= self.cards.len() {
            return;
        }
        let mut top = self.cards.split_off(index);
        top.append(&mut self.cards);
        self.cards = top;
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn display(&self) -> Vec<String> {
        self.cards.iter().map(|card| card.display()).collect()
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    pub fn deal_bottom(&mut self) -> Option<Card> {
        self.cards.pop_back()
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push_front(card);
    }

    pub fn add_card_bottom(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    pub fn peek(&self) -> Option<&Card> {
        self.cards.front()
    }

    pub fn peek_bottom(&self) -> Option<&Card> {
        self.cards.back()
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }
}

impl Add<Card> for Deck {
    type Output = Deck;

    fn add(mut self, rhs: Card) -> Deck {
        self.add_card(rhs);
        self
    }
}

impl Sub<Card> for Deck {
    type Output = Option<(Deck, Card)>;

    fn sub(mut self, rhs: Card) -> Option<(Deck, Card)> {
        if let Some(pos) = self.cards.iter().position(|&c| c == rhs) {
            let card = self.cards.remove(pos).unwrap();
            Some((self, card))
        }
        None
    }
}

impl Add<Deck> for Deck {
    type Output = Deck;

    fn add(mut self, rhs: Deck) -> Deck {
        for card in rhs.cards {
            self.add_card(card);
        }
        self
    }
}

impl Sub<Deck> for Deck {
    type Output = Deck;

    fn sub(mut self, rhs: Deck) -> Deck {
        for card in rhs.cards {
            if let Some(pos) = self.cards.iter().position(|&c| c == card) {
                self.cards.remove(pos);
            }
        }
        self
    }
}
