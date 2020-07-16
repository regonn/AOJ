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

const N: usize = 3;
const N2: usize = 9;

struct Puzzle {
    f: Vec<usize>,
    space: usize,
}

fn main() {
    let mut init_numbers:Vec<usize> = vec![0; N2];
    let mut init_space: usize = 0;
    for i in 0..N2 {
        let number: usize = read();
        init_numbers[i] =number;

        if init_numbers[i] == 0 {
            init_numbers[i] = N2;
            init_space = i;
        }
    }

    let mut puzzle = Puzzle{
        f: init_numbers,
        space: init_space
    };
}
