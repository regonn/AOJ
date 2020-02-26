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
    let s: u32 = read();
    let hours: u32 = s / (60 * 60);
    let hours_remaining: u32 = s % (60 * 60);
    let minites: u32 = hours_remaining / 60;
    let seconds: u32 = hours_remaining % 60;
    println!("{}:{}:{}", hours, minites, seconds);
}
