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
    let count: usize = read();
    let mut numbers: Vec<u16> = vec![];

    for _ in 0..count {
        let number: u16 = read();
        numbers.push(number);
    }

    for index in 1..count {
        for (i, number) in numbers.iter().enumerate() {
            if i != (numbers.len() - 1) {
                print!("{} ", number);
            } else {
                println!("{}", number);
            }
        }
        let v = numbers[index];
        let mut j = (index - 1) as i32;
        while j >= 0 && numbers[j as usize] > v {
            numbers[(j + 1) as usize] = numbers[j as usize];
            j -= 1;
        }
        numbers[(j + 1) as usize] = v;
    }

    for (i, number) in numbers.iter().enumerate() {
        if i != (numbers.len() - 1) {
            print!("{} ", number);
        } else {
            println!("{}", number);
        }
    }
}
