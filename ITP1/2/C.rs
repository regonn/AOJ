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
    let mut vec: Vec<u32> = Vec::new();
    for _ in 0..3 {
        vec.push(read());
    }
    vec.sort();
    println!("{} {} {}", vec[0], vec[1], vec[2])
}
