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
    let mut a: Vec<i64> = vec![];
    let mut index: usize = 0;

    for _ in 0..q {
        let query: i32 = read();
        match query {
            0 => {
                let number: i64 = read();
                a.insert(index, number);
            }
            1 => {
                let move_index: i64 = read();
                index = (index as i64 + move_index) as usize;
            }
            2 => {
                a.remove(index);
            }
            _ => (),
        }
    }

    for number in a {
        println!("{}", number);
    }
}
