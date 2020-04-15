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
    let mut v = VecDeque::new();
    for _ in 0..n {
        let command: String = read();
        match &*command {
            "insert" => {
                let value: u32 = read();
                v.push_front(value);
            }
            "delete" => {
                let value: u32 = read();
                match v.iter().position(|&x| x == value) {
                    Some(i) => {
                        v.remove(i);
                        ()
                    }
                    None => (),
                }
            }
            "deleteFirst" => {
                v.pop_front();
                ()
            }
            "deleteLast" => {
                v.pop_back();
                ()
            }
            _ => {}
        }
    }
    while !v.is_empty() {
        let value: u32 = v.pop_front().unwrap();
        if !v.is_empty() {
            print!("{} ", value);
        } else {
            println!("{}", value);
        }
    }
}
