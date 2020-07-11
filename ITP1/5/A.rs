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
        let h = read();
        let w = read();
        if h == 0 && w == 0 {
            done = true;
        } else {
            for _ in 0..h {
                for _ in 0..w {
                    print!("#");
                }
                println!();
            }
            println!();
        }
    }
}
