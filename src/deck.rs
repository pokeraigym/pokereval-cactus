use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, STR_RANKS};

pub struct Deck {
    cards: Vec<i32>,
}

impl Deck {
    pub fn shuffle(&mut self) -> () {
        self.cards = Deck::get_full_deck();
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn draw(&mut self, n: i32) -> Vec<i32> {
        let mut cards = Vec::new();

        for _ in 0..n {
            match self.cards.pop() {
                Some(card) => cards.push(card),
                None => return cards,
            }
        }
        cards
    }

    pub fn get_full_deck() -> Vec<i32> {
        let mut deck = Vec::new();

        for rank in STR_RANKS.chars() {
            let mut spade = rank.to_string();
            spade.push('s');
            let mut heart = rank.to_string();
            heart.push('h');
            let mut diamond = rank.to_string();
            diamond.push('d');
            let mut club = rank.to_string();
            club.push('c');

            deck.push(Card::new(spade));
            deck.push(Card::new(heart));
            deck.push(Card::new(diamond));
            deck.push(Card::new(club));
        }

        deck
    }
}
