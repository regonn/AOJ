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

fn gcd(x: u64, y: u64) -> u64 {
    let mut a: u64;
    let mut b: u64;

    if x < y {
        a = y;
        b = x;
    } else {
        a = x;
        b = y;
    }

    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    return a;
}

fn main() {
    let x: u64 = read();
    let y: u64 = read();
    println!("{}", gcd(x, y));
}
