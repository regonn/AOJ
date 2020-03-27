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
    let mut string: String = read();
    let count: u32 = read();

    for _ in 0..count {
        let command: String = read();
        let a: usize = read();
        let b: usize = read();

        if command == "replace" {
            let p: String = read();
            string = format!("{}{}{}", &string[..a], &p, &string[(b + 1)..]);
        } else if command == "print" {
            println!("{}", &string[a..(b + 1)]);
        } else {
            let mut chars: Vec<_> = string[a..(b + 1)].chars().collect();
            chars.reverse();

            let reversed_chars: String = chars.into_iter().collect();

            string = format!("{}{}{}", &string[..a], reversed_chars, &string[(b + 1)..]);
        }
    }
}
