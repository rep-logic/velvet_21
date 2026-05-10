mod cards;
mod display;

pub use self::cards::{Card, RANKS, Rank, SUITS, Suit};
use crate::errors::GameError;
use rand::prelude::*;
use std::fmt;

pub struct Deck<T> {
    cards: Vec<T>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PokerCard(Suit, Rank);

impl Deck<PokerCard> {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in SUITS {
            for rnk in RANKS {
                cards.push(PokerCard(suit, rnk));
            }
        }
        Self { cards }
    }
}

impl<T: Card + Clone + fmt::Debug> Deck<T> {
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }

    pub fn deal(&mut self) -> Result<T, GameError> {
        self.cards.pop().ok_or(GameError::EmptyDeck)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

impl Card for PokerCard {
    fn value(&self) -> u8 {
        match self.1 {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        }
    }

    fn display(&self) -> String {
        format!("{}", self)
    }
}
