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
    let mut p: Vec<u64> = vec![0; n + 1];
    let mut m: Vec<Vec<u64>> = vec![vec![u64::max_value(); n + 1]; n + 1];
    for index in 0..n {
        let r: u64 = read();
        let c: u64 = read();
        if index == 0 {
            p[index] = r;
            p[index + 1] = c;
        } else {
            p[index + 1] = c;
        }
    }
    for index in 1..(n + 1) {
        m[index][index] = 0;
    }
    for index_i in 2..(n + 1) {
        for index_j in 1..(n - index_i + 2) {
            let index_k = index_i + index_j - 1;
            for index_l in index_j..index_k {
                m[index_j][index_k] = min(
                    m[index_j][index_k],
                    m[index_j][index_l]
                        + m[index_l + 1][index_k]
                        + p[index_j - 1] * p[index_l] * p[index_k],
                );
            }
        }
    }
    println!("{}", m[1][n]);
}
