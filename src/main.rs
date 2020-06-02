use std::cmp::min;
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

fn main() {
    let n: usize = read();
    let mut p: Vec<u32> = vec![0; n + 1];
    let mut m: Vec<Vec<u32>> = vec![vec![0; n + 1]; n + 1];
    for index in 0..n {
        let r: u32 = read();
        let c: u32 = read();
        if index == 0 {
            p.push(r);
            p.push(c);
        } else {
            p.push(c);
        }
    }
    for index_i in 2..(n + 1) {
        for index_j in 1..(n - index_i) {
            let index_k = index_i + index_j - 1;
            m[index_j][index_k] = u32::max_value();
            for index_l in index_j..index_k {
                m[index_j][index_k] = min(m[index_j][index_l], 0)
            }
        }
    }
}
