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

fn main() {
    let mut done: bool = false;

    while !done {
        let mut cards: String = read();
        if cards == "-" {
            done = true;
        } else {
            let shuffle_number: u8 = read();
            for _ in 0..(shuffle_number) {
                let h: usize = read();
                cards = format!("{}{}", &cards[h..], &cards[..h]);
            }
            println!("{}", cards);
        }
    }
}
