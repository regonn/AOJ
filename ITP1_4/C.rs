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
    let mut done: bool = false;

    while !done {
        let a: i32 = read();
        let op: String = read();
        let op = op.as_bytes()[0] as char;
        let b: i32 = read();
        match op {
            '+' => println!("{}", a + b),
            '-' => println!("{}", a - b),
            '*' => println!("{}", a * b),
            '/' => println!("{}", a / b),
            _ => done = true,
        }
    }
}
