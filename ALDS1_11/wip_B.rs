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

const WHITE: u8 = 0;
const GRAY: u8 = 1;
const BLACK: u8 = 2;

fn main() {
    let n: usize = read();
    let mut color: Vec<u8> = vec![0; n];
    let mut stack: Vec<usize> = vec![];
    let mut m: Vec<Vec<u8>> = vec![vec![0; n]; n];
    for _ in 0..n {
        let index: usize = read();
        let count: usize = read();
        for _ in 0..count {
            let number: usize = read();
            m[index - 1][number - 1] = 1;
        }
    }
}
