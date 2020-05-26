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

fn left(node_id: usize) -> usize {
    return node_id * 2;
}

fn right(node_id: usize) -> usize {
    return node_id * 2 + 1;
}

fn max_heapify(a: &mut Vec<u32>, node_id: usize) {
    let left = left(node_id);
    let right = right(node_id);
    let mut largest: usize;
    if left <= (a.len() - 1) && a[left] > a[node_id] {
        largest = left;
    } else {
        largest = node_id;
    }
    if right <= (a.len() - 1) && a[right] > a[largest] {
        largest = right;
    }

    if largest != node_id {
        a.swap(node_id, largest);
        max_heapify(a, largest);
    }
}

fn build_max_heap(a: &mut Vec<u32>) {
    for i in (1..(((a.len() - 1) / 2) + 1)).rev() {
        max_heapify(a, i);
    }
}

fn main() {
    let mut done = false;
    let mut a: Vec<u32> = vec![0; 2_000_000];
    let mut h: usize = 0;
    while !done {
        let command: String = read();
        match &*command {
            "insert" => {
                let number: u32 = read();
                a.push(number);
                build_max_heap(&mut a);
            }
            "extract" => {
                let number: u32 = a.pop().unwrap();
                build_max_heap(&mut a);
                println!("{}", number);
            }
            _ => {
                done = true;
            }
        }
    }
}
