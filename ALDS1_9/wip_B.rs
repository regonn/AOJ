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

fn max_heapify(mut a: &Vec<i64>, node_id: usize) {}

fn build_max_heap(mut a: &Vec<i64>) {}

fn main() {
    let h: usize = read();
    let mut a: Vec<i64> = vec![0];
    for _ in 0..h {
        let number: i64 = read();
        a.push(number);
    }
    build_max_heap(&a);
}
