use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

struct Card {
    suit: char,
    number: u8,
}

fn main() {
    let mut cards: Vec<Card> = vec![];
    for suit in "SHCD".chars() {
        for number in 1..14 {
            let card = Card {
                suit: suit,
                number: number,
            };
            cards.push(card);
        }
    }

    let n: u8 = read();
    for _ in 0..n {
        let suit_str: String = read();
        let suit_char: char = suit_str.chars().next().unwrap();
        let number: u8 = read();
        cards.retain(|c| c.suit != suit_char || c.number != number);
    }

    for card in cards {
        println!("{} {}", card.suit, card.number)
    }
}
