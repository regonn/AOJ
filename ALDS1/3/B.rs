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
    let n: usize = read();
    let q: u16 = read();
    let mut time: u32 = 0;
    let mut v = VecDeque::new();
    for _ in 0..n {
        let s: String = read();
        let time: u16 = read();
        v.push_back((s, time))
    }

    while !v.is_empty() {
        let mut process: (String, u16) = v.pop_front().unwrap();
        if process.1 > q {
            time += q as u32;
            process.1 -= q;
            v.push_back(process)
        } else {
            time += process.1 as u32;
            println!("{} {}", process.0, time);
        }
    }
}
