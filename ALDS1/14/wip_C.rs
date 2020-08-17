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

fn check(fields: &Vec<Vec<char>>, target: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    return true;
}

fn main() {
    let h: usize = read();
    let w: usize = read();
    let mut fields: Vec<Vec<char>> = vec![vec!['0'; w]; h];
    for i in 0..h {
        let line_string: String = read();
        let chars: Vec<char> = line_string.chars().collect();
        for (index, char_data) in chars.iter().enumerate() {
            fields[i][index] = *char_data;
        }
    }
    let r: usize = read();
    let c: usize = read();
    let mut target: Vec<Vec<char>> = vec![vec!['0'; c]; r];
    for i in 0..r {
        let line_string: String = read();
        let chars: Vec<char> = line_string.chars().collect();
        for (index, char_data) in chars.iter().enumerate() {
            target[i][index] = *char_data;
        }
    }

    for i in 0..(h - r + 1) {
        for j in 0..(w - c + 1) {
            if check(&fields, &target, i,j){
                println!("{} {}", i, j);
            }
        }
    }
}
