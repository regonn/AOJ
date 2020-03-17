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
        let n: u32 = read();
        let x: u32 = read();
        let mut count: u32 = 0;
        if n == 0 && x == 0 {
            done = true;
        } else {
            for a in 1..(n + 1) {
                for b in (a + 1)..(n + 1) {
                    for c in (b + 1)..(n + 1) {
                        if (a + b + c) == x {
                            count += 1;
                        }
                    }
                }
            }
            println!("{}", count);
        }
    }
}
