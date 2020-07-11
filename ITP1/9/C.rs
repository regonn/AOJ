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

fn is_tarou_win(tarou_card: String, hanako_card: String) -> bool {
    let mut return_value: bool = false;
    let tarou_length = tarou_card.len();
    let hanako_length = hanako_card.len();
    for (index, tarou_char) in tarou_card.chars().enumerate() {
        let hanako_char: char = hanako_card[index..].chars().next().unwrap();
        if tarou_char != hanako_char {
            return_value = (tarou_char as u8) > (hanako_char as u8);
            break;
        }
        if (index + 1) == tarou_length || (index + 1) == hanako_length {
            return_value = tarou_length > hanako_length;
            break;
        }
    }
    return return_value;
}

fn main() {
    let count: u32 = read();
    let mut tarou_score: u32 = 0;
    let mut hanako_score: u32 = 0;

    for _ in 0..count {
        let tarou_card: String = read();
        let hanako_card: String = read();

        if tarou_card == hanako_card {
            tarou_score += 1;
            hanako_score += 1;
        } else if is_tarou_win(tarou_card, hanako_card) {
            tarou_score += 3;
        } else {
            hanako_score += 3;
        }
    }
    println!("{} {}", tarou_score, hanako_score);
}
