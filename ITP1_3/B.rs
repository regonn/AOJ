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
    let mut index: u32 = 1;

    while !done {
        let input: u32 = read();

        if input == 0 {
            done = true;
        } else {
            println!("Case {}: {}", index, input);
            index += 1;
        }
    }
}
