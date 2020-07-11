use std::ascii::AsciiExt;
use std::io::*;
use std::iter::FromIterator;
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

fn to_lower(c: char) -> char {
    c.to_ascii_lowercase()
}

fn main() {
    let w: String = read();
    let mut done: bool = false;
    let mut count: u32 = 0;

    while !done {
        let t: String = read();
        let t_lower: String = String::from_iter(t.chars().map(to_lower).into_iter());
        if t == "END_OF_TEXT" {
            done = true;
        } else if t_lower == w {
            count += 1;
        }
    }
    println!("{}", count);
}
