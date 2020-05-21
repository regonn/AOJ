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

fn parent(node_id: usize) -> usize {
    return node_id / 2;
}

fn main() {
    let h: usize = read();
    let a: Vec<i64> = (0..h)
        .map(|_| {
            let number: i64 = read();
            return number;
        })
        .collect();
    for (index, node_key) in a.iter().enumerate() {
        print!("node {}: key = {}", index + 1, node_key);
    }
}
