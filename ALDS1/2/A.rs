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
    let mut a: Vec<u8> = vec![];
    let mut bubble_count: u16 = 0;
    let mut bubble_flag: bool = true;
    for _ in 0..n {
        let number: u8 = read();
        a.push(number);
    }

    while bubble_flag {
        bubble_flag = false;
        for index in (1..n).rev() {
            if a[index] < a[index - 1] {
                bubble_flag = true;
                bubble_count += 1;
                a.swap(index, index - 1);
            }
        }
    }

    for (index, number) in a.iter().enumerate() {
        if index != a.len() - 1 {
            print!("{} ", number);
        } else {
            println!("{}", number);
        }
    }
    println!("{}", bubble_count);
}
