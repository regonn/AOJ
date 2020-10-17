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
    let q: usize = read();
    let mut a: Vec<Vec<i64>> = vec![vec![]; n];

    for _ in 0..q {
        let query: u8 = read();
        let t: usize = read();
        match query {
            0 => {
                let x: i64 = read();
                a[t].push(x);
            }
            1 => {
                let length = a[t].len();
                for index in 0..length {
                    if index == length - 1 {
                        print!("{}", a[t][index])
                    } else {
                        print!("{} ", a[t][index])
                    }
                }
                println!()
            }
            2 => {
                a[t].clear();
            }
            _ => (),
        }
    }
}
