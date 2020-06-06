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

fn lcs(x: Vec<char>, y: Vec<char>) -> u16 {
    let mut matrix: Vec<Vec<u16>> = vec![vec![0; y.len()]; x.len()];
    let mut input_number = 0;
    for i in 0..x.len() {
        if x[i] == y[0] {
            input_number = 1;
        }
        matrix[i][0] = input_number;
    }
    return matrix[x.len() - 1][y.len() - 1];
}

fn main() {
    let q: usize = read();
    for _ in 0..q {
        let x_string: String = read();
        let y_string: String = read();
        println!(
            "{}",
            lcs(x_string.chars().collect(), y_string.chars().collect())
        );
    }
}
