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
    let a: u32 = read();
    let b: u32 = read();
    let c: u32 = read();
    let mut count: u32 = 0;

    for number in a..(b+1) {
        if c % number == 0 {
            count += 1;
        }
    }
    println!("{}", count);
}
