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
    let mut vec: Vec<u32> = Vec::new();
    let mut first_flag: bool = true;
    for _ in 0..n {
        vec.push(read());
    }
    vec.reverse();
    for number in vec {
        if first_flag {
            print!("{}", number);
            first_flag = false;
        } else {
            print!(" {}", number);
        }
    }
    println!();
}
