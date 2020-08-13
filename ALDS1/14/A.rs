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
    let t: String = read();
    let p: String = read();
    if t.len() < p.len() {
        return;
    }
    for index in 0..(t.len() - p.len() + 1) {
        if &t[index..index + p.len()] == p {
            println!("{}", index);
        }
    }
}
