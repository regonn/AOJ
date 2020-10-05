use std::collections::VecDeque;
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
    let q: usize = read();
    let mut a = VecDeque::new();
    for _ in 0..q {
        let query: i32 = read();
        match query {
            0 => {
                let back: usize = read();
                let number: i32 = read();
                if back == 0 {
                    a.push_front(number);
                } else {
                    a.push_back(number);
                }
            }
            1 => {
                let index: usize = read();
                println!("{}", a[index]);
            }
            2 => {
                let back: usize = read();
                if back == 0 {
                    let _ = a.pop_front();
                } else {
                    let _ = a.pop_back();
                }
            }
            _ => (),
        }
    }
}
