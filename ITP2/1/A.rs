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
    let q: usize = read();
    let mut a: Vec<i32> = vec![];
    for _ in 0..q {
        let query: i32 = read();
        match query {
            0 => {
                let number: i32 = read();
                a.push(number);
            }
            1 => {
                let index: usize = read();
                println!("{}", a[index]);
            }
            2 => {
                let _: i32 = a.pop().unwrap();
            }
            _ => (),
        }
    }
}
