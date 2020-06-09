use std::cmp::max;
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
    let x_len = x.len();
    let y_len = y.len();
    let mut matrix: Vec<Vec<u16>> = vec![vec![0; y_len]; x_len];
    let mut input_number = 0;

    for i in 0..(x_len) {
        if x[i] == y[0] {
            input_number = 1;
        }
        matrix[i][0] = input_number;
    }
    input_number = 0;
    for j in 0..(y_len) {
        if x[0] == y[j] {
            input_number = 1;
        }
        matrix[0][j] = input_number;
    }

    for i in 1..(x_len) {
        for j in 1..(y_len) {
            if x[i] == y[j] {
                matrix[i][j] = matrix[i - 1][j - 1] + 1
            } else {
                matrix[i][j] = max(matrix[i][j - 1], matrix[i - 1][j])
            }
        }
    }
    return matrix[x_len - 1][y_len - 1];
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
