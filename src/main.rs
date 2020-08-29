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
    let mut n: u32 = read();
    let mut count: usize = 0;

    while n >= 25 {
        n -= 25;
        count += 1;
    }

    while n >= 10 {
        n -= 10;
        count += 1;
    }

    while n >= 5 {
        n -= 5;
        count += 1;
    }

    while n >= 1 {
        n -= 1;
        count += 1;
    }

    println!("{}", count);
}
