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

fn left(node_id: usize) -> usize {
    return node_id * 2;
}

fn right(node_id: usize) -> usize {
    return node_id * 2 + 1;
}

fn main() {
    let h: usize = read();
    let mut a: Vec<i64> = vec![0];
    for _ in 0..h {
        let number: i64 = read();
        a.push(number);
    }
    for (index, node_key) in a.iter().enumerate() {
        if index == 0 {
            continue;
        }
        print!("node {}: key = {}, ", index, node_key);
        if parent(index) >= 1 {
            print!("parent key = {}, ", a[parent(index)]);
        }
        if left(index) <= h {
            print!("left key = {}, ", a[left(index)]);
        }
        if right(index) <= h {
            print!("right key = {}, ", a[right(index)]);
        }
        println!();
    }
}
