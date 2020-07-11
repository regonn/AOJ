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
    let w: i32 = read();
    let h: i32 = read();
    let x: i32 = read();
    let y: i32 = read();
    let r: i32 = read();

    if y >= r && x >= r && (x + r) <= w && (y + r) <= h {
        println!("Yes");
    } else {
        println!("No");
    }
}
