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

fn binary_search(array: Vec<u32>, key: u32) -> usize {
    let len: usize = array.len();
    let mut left: usize = 0;
    let mut right: usize = len;
    while left < right {
        let mid: usize = (left + right) / 2;
        if array[mid] == key {
            return mid;
        } else if key < array[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return len;
}

fn main() {
    let n: usize = read();
    let s: Vec<u32> = (0..n)
        .map(|_| {
            let number: u32 = read();
            number
        })
        .collect();
    let q: usize = read();
    let t: Vec<u32> = (0..q)
        .map(|_| {
            let number: u32 = read();
            number
        })
        .collect();
    let mut count: u32 = 0;
    let s_len: usize = s.len();
    for key in t {
        if s_len != binary_search(s.clone(), key) {
            count += 1;
        }
    }
    println!("{}", count);
}
