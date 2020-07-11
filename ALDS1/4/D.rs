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

fn check(n: u32, k: u32, p: u32, w: Vec<u32>) -> u32 {
    let mut i: usize = 0;
    let mut j: u32 = 0;
    while j < k {
        let mut s: u32 = 0;
        while s + w[i] <= p {
            s += w[i];
            i += 1;
            if i as u32 == n {
                return n;
            }
        }
        j += 1;
    }
    return i as u32;
}

fn main() {
    let n: u32 = read();
    let k: u32 = read();
    let w: Vec<u32> = (0..n)
        .map(|_| {
            let number: u32 = read();
            number
        })
        .collect();
    let mut left: u32 = 0;
    let mut right: u32 = 100_000 * 10000;
    let mut mid: u32;
    while right - left > 1 {
        mid = (right + left) / 2;
        if check(n, k, mid, w.clone()) >= n {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right);
}
