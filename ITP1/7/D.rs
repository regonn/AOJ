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
    let m: usize = read();
    let l: usize = read();
    let mut a: Vec<Vec<u64>> = vec![vec![0; m]; n];
    let mut b: Vec<Vec<u64>> = vec![vec![0; l]; m];
    let mut c: Vec<Vec<u64>> = vec![vec![0; l]; n];
    for n_index in 0..n {
        for m_index in 0..m {
            a[n_index][m_index] = read();
        }
    }
    for m_index in 0..m {
        for l_index in 0..l {
            b[m_index][l_index] = read();
        }
    }
    for n_index in 0..n {
        for l_index in 0..l {
            for m_index in 0..m {
                c[n_index][l_index] += a[n_index][m_index] * b[m_index][l_index]
            }
            if l_index != (l - 1) {
                print!("{} ", c[n_index][l_index])
            } else {
                println!("{}", c[n_index][l_index])
            }
        }
    }
}
