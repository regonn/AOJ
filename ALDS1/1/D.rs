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
    let n: u64 = read();
    let mut minv: i64 = read();
    let second_v: i64 = read();
    let mut maxv: i64 = second_v - minv;
    if minv > second_v {
        minv = second_v;
    }
    for _ in 2..n {
        let v: i64 = read();
        if maxv < v - minv {
            maxv = v - minv;
        }
        if minv > v {
            minv = v;
        }
    }
    println!("{}", maxv);
}
