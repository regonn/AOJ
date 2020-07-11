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
    let r: usize = read();
    let c: usize = read();
    let mut array: Vec<Vec<u32>> = vec![vec![0; c + 1]; r + 1];
    for r_index in 0..(r as u32 + 1) {
        if r_index != (r as u32) {
            let mut sum_row: u32 = 0;
            for c_index in 0..(c as u32 + 1) {
                if c_index != (c as u32) {
                    let number: u32 = read();
                    array[r_index as usize][c_index as usize] = number;
                    print!("{} ", number);
                    sum_row += number;
                } else {
                    println!("{}", sum_row);
                    array[r_index as usize][c_index as usize] = sum_row;
                }
            }
        } else {
            for c_index in 0..(c as u32 + 1) {
                let mut sum_col: u32 = 0;
                for r_index_2 in 0..(r as u32) {
                    sum_col += array[r_index_2 as usize][c_index as usize]
                }
                if c_index != (c as u32) {
                    print!("{} ", sum_col);
                } else {
                    println!("{}", sum_col);
                }
            }
        }
    }
}
