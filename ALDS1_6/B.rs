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

fn partition(a: &mut Vec<u32>) -> usize {
    let mut i: usize = 0;
    let x = *a.last().unwrap();

    for j in 0..a.len() {
        if a[j] <= x {
            a.swap(i, j);
            i += 1;
        }
    }
    return i - 1;
}

fn main() {
    let n: usize = read();
    let mut a: Vec<u32> = (0..n)
        .map(|_| {
            let number: u32 = read();
            number
        })
        .collect();
    let partition_index: usize = partition(&mut a);
    for (index, number) in a.iter().enumerate() {
        if (index + 1) != a.len() {
            if index == partition_index {
                print!("[{}] ", number);
            } else {
                print!("{} ", number);
            }
        } else {
            println!("{}", number);
        }
    }
}
