use std::cmp::Ordering;
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
        let x: u32 = read();
        let y: u32 = read();

        if x == 0 && y == 0 {
            done = true;
        } else {
            match x.cmp(&y) {
                Ordering::Less => println!("{} {}", x, y),
                Ordering::Greater => println!("{} {}", y, x),
                Ordering::Equal => println!("{} {}", x, y),
            };
        }
    }
}
