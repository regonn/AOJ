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
    let a: u64 = read();
    let b: u64 = read();
    let a_float: f64 = a as f64;
    let b_float: f64 = b as f64;
    println!("{} {} {}", (a / b), (a % b), (a_float / b_float));
}
