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
        let m: i8 = read();
        let f: i8 = read();
        let r: i8 = read();
        if m != -1 || f != -1 {
            if m == -1 || f == -1 {
                println!("F");
            } else {
                let score: i8 = m + f;
                if score >= 80 {
                    println!("A");
                } else if score >= 65 {
                    println!("B");
                } else if score >= 50 {
                    println!("C");
                } else if score >= 30 {
                    if r >= 50 {
                        println!("C");
                    } else {
                        println!("D")
                    }
                } else {
                    println!("F")
                }
            }
        } else {
            if r != -1 {
                println!("F");
            } else {
                done = true;
            }
        }
    }
}
