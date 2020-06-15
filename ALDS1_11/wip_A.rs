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
    // TODO: CASE#4
    let n: usize = read();
    let mut a: Vec<Vec<u8>> = vec![vec![0; n]; n];
    for _ in 0..n {
        let index: usize = read();
        let count: usize = read();
        for _ in 0..count {
            let number: usize = read();
            if number > index {
                a[index - 1][number - 1] = 1;
            } else {
                if a[number - 1][index - 1] == 1 {
                    a[index - 1][number - 1] = 0
                } else {
                    a[index - 1][number - 1] = 1
                }
            }
        }
    }
    for index_i in 0..n {
        for index_j in 0..n {
            if index_j == n - 1 {
                println!("{}", a[index_i][index_j]);
            } else {
                print!("{} ", a[index_i][index_j]);
            }
        }
    }
}
