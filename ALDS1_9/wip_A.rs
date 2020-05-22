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
    let a: Vec<i64> = (1..h)
        .map(|_| {
            let number: i64 = read();
            return number;
        })
        .collect();
    for (index, node_key) in a.iter().enumerate() {
        let current_node_id = index + 1;
        print!("node {}: key = {}, ", current_node_id, node_key);
        if parent(current_node_id) >= 1 {
            print!("parent key = {}, ", a[parent(current_node_id)]);
        }
        if left(current_node_id) < h {
            print!("left key = {}, ", a[left(current_node_id)]);
        }
        if right(current_node_id) < h {
            print!("right key = {}, ", a[right(current_node_id)]);
        }
        println!();
    }
}
