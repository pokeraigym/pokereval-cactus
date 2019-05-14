pub(crate) const STR_RANKS: &str = "23456789TJQKA";
pub(crate) const PRIMES: &[i32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
pub(crate) const INT_RANKS: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
const INT_SUIT_TO_CHAR_SUIT: &str = "xshxdxxxc";

fn char_rank_to_int_rank(rank: char) -> i32 {
    match rank {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!(),
    }
}

fn char_suit_to_int_suit(suit: char) -> i32 {
    match suit {
        's' => 1, // spades
        'h' => 2, // hearts
        'd' => 4, // diamonds
        'c' => 8, // clubs
        _ => unreachable!(),
    }
}

fn pretty_suits(suit: i32) -> char {
    match suit {
        1 => '♠',
        2 => '♥',
        4 => '♦',
        8 => '♣',
        _ => unreachable!(),
    }
}

pub struct Card{}

impl Card {
    pub fn new(string: String) -> i32 {
        let rank_char = string.chars().nth(0).unwrap();
        let suit_char = string.chars().nth(1).unwrap();
        let rank_int = char_rank_to_int_rank(rank_char);
        let suit_int = char_suit_to_int_suit(suit_char);
        let rank_prime = PRIMES[rank_int as usize];

        let bitrank = 1 << rank_int << 16;
        let suit = suit_int << 12;
        let rank = rank_int << 8;

        bitrank | suit | rank | rank_prime
    }

    pub fn int_to_str(card_int: i32) -> String {
        let rank_int = Card::get_rank_int(card_int) as usize;
        let suit_int = Card::get_suit_int(card_int) as usize;
        format!(
            "{}{}",
            STR_RANKS.chars().nth(rank_int).unwrap(),
            INT_SUIT_TO_CHAR_SUIT.chars().nth(suit_int).unwrap()
        )
    }

    pub fn get_rank_int(card_int: i32) -> i32 {
        (card_int >> 8) & 0xF
    }

    pub fn get_suit_int(card_int: i32) -> i32 {
        (card_int >> 12) & 0xF
    }

    pub fn get_bitrank_int(card_int: i32) -> i32 {
        (card_int >> 16) & 0x1FFF
    }

    pub fn get_prime(card_int: i32) -> i32 {
        card_int & 0x3F
    }

    pub fn hand_to_binary(card_strs: Vec<String>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        for c in card_strs {
            ret.push(Card::new(c));
        }
        ret
    }

    pub fn prime_product_from_hand(card_ints: Vec<i32>) -> i32 {
        let mut product = 1;
        for c in card_ints {
            product *= c & 0xFF;
        }
        product
    }

    pub fn prime_product_from_rankbits(rankbits: i32) -> i32 {
        let mut product = 1;
        for i in INT_RANKS {
            if rankbits & (1 << i) != 0 {
                product *= PRIMES[*i as usize];
            }
        }
        product
    }

    pub fn int_to_pretty_str(card_int: i32) -> String {
        let suit_int = Card::get_suit_int(card_int);
        let rank_int = Card::get_rank_int(card_int);
        let s = pretty_suits(suit_int);
        let r = STR_RANKS.chars().nth(rank_int as usize).unwrap();

        format!("{}{}", r, s)
    }

    pub fn print_pretty_card(card_int: i32) -> String {
        Card::int_to_pretty_str(card_int)
    }

    pub fn print_pretty_cards(card_ints: Vec<i32>) -> String {
        let mut output = String::from(" ");

        for i in 0..card_ints.len() {
            let c = card_ints[i];
            if i != card_ints.len() - 1 {
                output.push_str(&(Card::int_to_pretty_str(c) + ","));
            } else {
                output.push_str(&(Card::int_to_pretty_str(c) + " "));
            }
        }
        output
    }
}