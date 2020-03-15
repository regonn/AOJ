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

    let mut a: Vec<Vec<u32>> = vec![vec![0; m]; n];
    let mut b: Vec<u32> = vec![0; m];

    for index_n in 0..n {
        for index_m in 0..m {
            a[index_n][index_m] = read();
        }
    }

    for index_m in 0..m {
        b[index_m] = read();
    }

    for index_n in 0..n {
        let mut sum: u32 = 0;
        for index_m in 0..m {
            sum += a[index_n][index_m] * b[index_m];
        }
        println!("{}", sum);
    }
}
