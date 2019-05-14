use crate::card::Card;
use crate::lookup::{rank_class_to_string, LookupTable, MAX_HIGH_CARD};
use itertools::Itertools;

pub struct Evaluator {
    table: LookupTable,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            table: LookupTable::new(),
        }
    }

    pub fn evaluate(&self, mut cards: Vec<i32>, mut board: Vec<i32>) -> i32 {
        cards.append(&mut board);
        let length = cards.len();
        let value = match length {
            5 => self.five(cards),
            6 => self.six(cards),
            7 => self.seven(cards),
            _ => unreachable!(),
        };

        value
    }

    pub fn class_to_string(&self, class_int: i32) -> String {
        rank_class_to_string(class_int)
    }
    fn five(&self, cards: Vec<i32>) -> i32 {
        if (cards[0] & cards[1] & cards[2] & cards[3] & cards[4] & 0xF000) != 0 {
            let hand_or = (cards[0] | cards[1] | cards[2] | cards[3] | cards[4]) >> 16;
            let prime = Card::prime_product_from_rankbits(hand_or);
            self.table.flush_lookup[&prime]
        } else {
            let prime = Card::prime_product_from_hand(cards);
            self.table.unsuited_lookup[&prime]
        }
    }

    fn six(&self, cards: Vec<i32>) -> i32 {
        let mut minimum = MAX_HIGH_CARD;

        let all5cardcombos = cards.iter().combinations(5);

        for combo in all5cardcombos {
            let score = self.five(vec![*combo[0], *combo[1], *combo[2], *combo[3], *combo[4]]);
            if score < minimum {
                minimum = score;
            }
        }

        minimum
    }

    fn seven(&self, cards: Vec<i32>) -> i32 {
        self.six(cards)
    }
}
