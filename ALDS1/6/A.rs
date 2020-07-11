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

fn counting_sort(a: &Vec<usize>) -> Vec<usize> {
    let mut count = vec![0; 10001];
    for &index in a.iter() {
        count[index] += 1;
    }

    for index in 1..count.len() {
        count[index] += count[index - 1]
    }

    let mut b = vec![0; a.len()];
    for &index in a.iter().rev() {
        b[count[index] -1] = index;
        count[index] -= 1;
    }

    return b;
}

fn main() {
    let n: usize = read();
    let a: Vec<usize> = (0..n)
        .map(|_| {
            let number: usize = read();
            number
        })
        .collect();
    let b = counting_sort(&a);
    for (index, number) in b.iter().enumerate() {
        if (index + 1) != b.len() {
            print!("{} ", number);
        } else {
            println!("{}", number);
        }
    }
}
