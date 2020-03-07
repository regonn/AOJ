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
    let n: u32 = read();
    let first_number: i64 = read();
    let mut min: i64 = first_number;
    let mut max: i64 = first_number;
    let mut sum: i64 = first_number;

    for _ in 0..(n - 1) {
        let number: i64 = read();
        if number < min {
            min = number;
        } else if number > max {
            max = number;
        }
        sum += number;
    }
    println!("{} {} {}", min, max, sum);
}
