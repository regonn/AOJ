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

fn is_prime(x: u64) -> bool {
    if x == 2 {
        return true;
    }

    if x < 2 || x % 2 == 0 {
        return false;
    }

    let mut index: u64 = 3;
    let x_sqrt: u64 = (x as f64).sqrt() as u64;
    while index <= x_sqrt {
        if x % index == 0 {
            return false;
        }
        index += 2;
    }
    return true;
}

fn main() {
    let n: u64 = read();
    let mut count: u64 = 0;
    for _ in 0..n {
        let number: u64 = read();
        if is_prime(number) {
            count += 1;
        }
    }
    println!("{}", count);
}
