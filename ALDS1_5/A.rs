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

fn solve(i: usize, m: i16, a: &Vec<i16>) -> bool {
    if m == 0 {
        return true;
    }
    if i >= a.len() {
        return false;
    }
    return solve(i + 1, m, a) || solve(i + 1, m - a[i], a);
}

fn main() {
    let n: usize = read();
    let a: Vec<i16> = (0..n)
        .map(|_| {
            let number: i16 = read();
            number
        })
        .collect();
    let q: usize = read();
    let m: Vec<i16> = (0..q)
        .map(|_| {
            let number: i16 = read();
            number
        })
        .collect();
    for m_number in m {
        if solve(0, m_number, &a) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
