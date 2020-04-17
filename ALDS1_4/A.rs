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

fn linear_search(array: Vec<u32>, key: u32) -> usize {
    let mut cloned_array: Vec<u32> = array;
    let mut index: usize = 0;
    cloned_array.push(key);
    while cloned_array[index] != key {
        index += 1;
    }
    return index;
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
        if s_len != linear_search(s.clone(), key) {
            count += 1;
        }
    }
    println!("{}", count);
}
