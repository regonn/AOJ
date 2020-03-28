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
    let x1: f64 = read();
    let y1: f64 = read();
    let x2: f64 = read();
    let y2: f64 = read();
    println!("{}", ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt());
}
