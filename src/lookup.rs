use itertools::Itertools;

use std::collections::HashMap;

use crate::card::{Card, PRIMES};

const MAX_STRAIGHT_FLUSH: i32 = 10;
const MAX_FOUR_OF_A_KIND: i32 = 166;
const MAX_FULL_HOUSE: i32 = 322;
const MAX_FLUSH: i32 = 1599;
const MAX_STRAIGHT: i32 = 1609;
const MAX_THREE_OF_A_KIND: i32 = 2467;
const MAX_TWO_PAIR: i32 = 3325;
const MAX_PAIR: i32 = 6185;
pub(crate) const MAX_HIGH_CARD: i32 = 7462;

pub fn rank_class_to_string(rank: i32) -> String {
    match rank {
        1 => String::from("Straight Flush"),
        2 => String::from("Four of a Kind"),
        3 => String::from("Full House"),
        4 => String::from("Flush"),
        5 => String::from("Straight"),
        6 => String::from("Three of a Kind"),
        7 => String::from("Two Pair"),
        8 => String::from("Pair"),
        9 => String::from("High Card"),
        _ => unreachable!(),
    }
}

pub struct LookupTable {
    pub flush_lookup: HashMap<i32, i32>,
    pub unsuited_lookup: HashMap<i32, i32>,
}

struct StanfordBitIterator {
    bits: i32,
}

impl Iterator for StanfordBitIterator {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let t = (self.bits | (self.bits - 1)) + 1;
        let next_value = t | ((((t & -t) / (self.bits & -self.bits)) >> 1) - 1);
        self.bits = next_value;
        Some(next_value)
    }
}

fn get_lexographically_next_bit_sequence(bits: i32) -> StanfordBitIterator {
    StanfordBitIterator { bits }
}

fn flushes() -> (HashMap<i32, i32>, [i32; 10], Vec<i32>) {
    let mut flush_lookup = HashMap::new();

    let straight_flushes = [
        7936, // int('0b1111100000000', 2), # royal flush
        3968, // int('0b111110000000', 2),
        1984, // int('0b11111000000', 2),
        992,  // int('0b1111100000', 2),
        496,  // int('0b111110000', 2),
        248,  // int('0b11111000', 2),
        124,  // int('0b1111100', 2),
        62,   // int('0b111110', 2),
        31,   // int('0b11111', 2),
        4111, // int('0b1000000001111', 2) # 5 high
    ];

    let mut flushes: Vec<i32> = Vec::new();
    let mut gen = get_lexographically_next_bit_sequence(0b11111);

    for _ in 0..(1277 + straight_flushes.len() - 1) {
        let f = gen.next().unwrap();
        let mut not_sf = true;
        for sf in straight_flushes.iter() {
            if f ^ sf == 0 {
                not_sf = false;
            }
        }

        if not_sf {
            flushes.push(f);
        }
    }
    flushes.reverse();
    let mut rank = 1;
    for sf in straight_flushes.iter() {
        let prime_product = Card::prime_product_from_rankbits(*sf);
        flush_lookup.insert(prime_product, rank);
        rank += 1;
    }
    // works up to here
    rank = MAX_FULL_HOUSE + 1;
    for f in flushes.iter() {
        let prime_product = Card::prime_product_from_rankbits(*f);
        flush_lookup.insert(prime_product, rank);
        rank += 1;
    }

    (flush_lookup, straight_flushes, flushes)
}

fn unsuited_lookups(straights: [i32; 10], highcards: Vec<i32>) -> HashMap<i32, i32> {
    let mut unsuited_lookup = HashMap::new();

    // straight and highcards
    let mut rank = MAX_FLUSH + 1;

    for &s in straights.iter() {
        let prime_product = Card::prime_product_from_rankbits(s);
        unsuited_lookup.insert(prime_product, rank);
        rank += 1
    }

    rank = MAX_PAIR + 1;
    for &h in highcards.iter() {
        let prime_product = Card::prime_product_from_rankbits(h);
        unsuited_lookup.insert(prime_product, rank);
        rank += 1;
    }

    // multiples
    let backwards_ranks: Vec<i32> = vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]; // equal to backwards_ranks from treys
    rank = MAX_STRAIGHT_FLUSH + 1;

    for &i in backwards_ranks.iter() {
        let mut kickers = backwards_ranks.clone();
        kickers
            .iter()
            .position(|&n| n == i)
            .map(|e| kickers.remove(e));
        for &k in kickers.iter() {
            let product = PRIMES[i as usize].pow(4) * PRIMES[k as usize];
            unsuited_lookup.insert(product, rank);
            rank += 1;
        }
    }

    rank = MAX_FOUR_OF_A_KIND + 1;
    for &i in backwards_ranks.iter() {
        let mut pairranks = backwards_ranks.clone();
        pairranks
            .iter()
            .position(|&n| n == i)
            .map(|e| pairranks.remove(e));
        for &pr in pairranks.iter() {
            let product = PRIMES[i as usize].pow(3) * PRIMES[pr as usize].pow(2);
            unsuited_lookup.insert(product, rank);
            rank += 1;
        }
    }
    rank = MAX_STRAIGHT + 1;

    for &r in backwards_ranks.iter() {
        let mut kickers = backwards_ranks.clone();
        kickers
            .iter()
            .position(|&n| n == r)
            .map(|e| kickers.remove(e));
        let gen = kickers.iter().combinations(2);

        for kickers in gen {
            let c1 = *kickers[0];
            let c2 = *kickers[1];
            let product = PRIMES[r as usize].pow(3) * PRIMES[c1 as usize] * PRIMES[c2 as usize];
            unsuited_lookup.insert(product, rank);
            rank += 1;
        }
    }

    rank = MAX_THREE_OF_A_KIND + 1;

    let tpgen = backwards_ranks.iter().combinations(2);

    for tp in tpgen {
        let pair1 = *tp[0];
        let pair2 = *tp[1];
        let mut kickers = backwards_ranks.clone();
        kickers
            .iter()
            .position(|&n| n == pair1)
            .map(|e| kickers.remove(e));
        kickers
            .iter()
            .position(|&n| n == pair2)
            .map(|e| kickers.remove(e));

        for &kicker in kickers.iter() {
            let product = PRIMES[pair1 as usize].pow(2)
                * PRIMES[pair2 as usize].pow(2)
                * PRIMES[kicker as usize];
            unsuited_lookup.insert(product, rank);
            rank += 1;
        }
    }

    rank = MAX_TWO_PAIR + 1;

    for &pairrank in backwards_ranks.iter() {
        let mut kickers = backwards_ranks.clone();
        kickers
            .iter()
            .position(|&n| n == pairrank)
            .map(|e| kickers.remove(e));
        let kgen = kickers.iter().combinations(3);

        for kickers in kgen {
            let k1 = *kickers[0];
            let k2 = *kickers[1];
            let k3 = *kickers[2];

            let product = PRIMES[pairrank as usize].pow(2)
                * PRIMES[k1 as usize]
                * PRIMES[k2 as usize]
                * PRIMES[k3 as usize];
            unsuited_lookup.insert(product, rank);
            rank += 1;
        }
    }
    unsuited_lookup
}

impl LookupTable {
    pub fn new() -> Self {
        let flush_results = flushes();
        let flush_hash_map = flush_results.0;
        LookupTable {
            flush_lookup: flush_hash_map,
            unsuited_lookup: unsuited_lookups(flush_results.1, flush_results.2),
        }
    }
}