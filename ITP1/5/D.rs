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

fn include3(i: i32, x: i32) {
    if x % 10 == 3 {
        print!(" {}", i);
        return;
    }
    let x_next: i32 = x / 10;
    if x_next != 0 {
        include3(i, x_next);
    }
}

fn check_num(i: i32, x: i32) {
    if x % 3 == 0 {
        print!(" {}", i);
        return;
    }
    include3(i, x);
}

fn main() {
    let n: i32 = read();
    let mut i: i32 = 1;

    while i <= n {
        let x: i32 = i;
        check_num(i, x);
        i += 1;
    }
    println!();
}
