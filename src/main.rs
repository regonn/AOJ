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
    let mut p: Vec<u32> = vec![0];
    let mut m: Vec<Vec<u32>> = vec![vec![]];
    for index in 0..n {
        let r: u32 = read();
        let c: u32 = read();
        if index == 0 {
            p.push(r);
            p.push(c);
        } else {
            p.push(c);
        }
    }
}
