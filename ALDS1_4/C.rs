use std::collections::HashSet;
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
    let mut dict = HashSet::new();
    for _ in 0..n {
        let action: String = read();
        let word: String = read();
        if action == "insert" {
            dict.insert(word);
        } else {
            if dict.contains(&word) {
                println!("yes");
            } else {
                println!("no");
            }
        }
    }
}
