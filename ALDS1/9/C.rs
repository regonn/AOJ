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

fn max_heapify(a: &mut Vec<i32>, node_id: usize) {
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

fn increase_key(key: i32, a: &mut Vec<i32>) {
    let mut i = a.len() - 1;
    if key < a[i] {
        return;
    }
    a[i] = key;
    while i > 1 && a[i / 2] < a[i] {
        a.swap(i, i / 2);
        i = i / 2;
    }
}

fn insert(key: i32, a: &mut Vec<i32>) {
    a.push(i32::min_value());
    increase_key(key, a);
}

fn extract(a: &mut Vec<i32>) -> i32 {
    if a.len() < 1 {
        return i32::min_value();
    }

    let maxv;
    maxv = a[1];
    a[1] = *a.last().unwrap();
    a.pop();
    max_heapify(a, 1);
    maxv
}

fn main() {
    let mut done = false;
    let mut a: Vec<i32> = vec![-1];

    while !done {
        let command: String = read();
        match &*command {
            "insert" => {
                let number: i32 = read();
                insert(number, &mut a);
            }
            "extract" => println!("{}", extract(&mut a)),
            _ => {
                done = true;
            }
        }
    }
}
