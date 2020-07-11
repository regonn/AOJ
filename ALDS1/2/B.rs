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
    let mut swap_count: u16 = 0;

    for _ in 0..n {
        let number: u8 = read();
        a.push(number);
    }

    for index in 0..n {
        let mut min_j = index;
        for j in index..n {
            if a[j] < a[min_j] {
                min_j = j
            }
        }
        if index != min_j {
            a.swap(index, min_j);
            swap_count += 1;
        }
    }

    for (index, number) in a.iter().enumerate() {
        if index != a.len() - 1 {
            print!("{} ", number);
        } else {
            println!("{}", number);
        }
    }
    println!("{}", swap_count);
}
