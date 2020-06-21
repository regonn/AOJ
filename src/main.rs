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

const WHITE: u8 = 0;
const GRAY: u8 = 1;
const BLACK: u8 = 2;

fn dfs_visit(
    m: &Vec<Vec<u8>>,
    index: usize,
    color: &mut Vec<u8>,
    time: &mut u32,
    d: &mut Vec<u32>,
    f: &mut Vec<u32>,
) {
    color[index] = GRAY;
    *time = *time + 1;
    d[index] = *time;
    for index_target in 0..color.len() {
        if m[index][index_target] == 0 {
            continue;
        }
        if color[index_target] == WHITE {
            dfs_visit(m, index_target, color, time, d, f);
        }
    }
    color[index] = BLACK;
    *time = *time + 1;
    f[index] = *time;
}

fn main() {
    let n: usize = read();
    let mut color: Vec<u8> = vec![WHITE; n];
    let mut m: Vec<Vec<u8>> = vec![vec![0; n]; n];
    let mut d: Vec<u32> = vec![0; n];
    let mut f: Vec<u32> = vec![0; n];
    let mut time: u32 = 0;
    for _ in 0..n {
        let index: usize = read();
        let count: usize = read();
        for _ in 0..count {
            let number: usize = read();
            m[index - 1][number - 1] = 1;
        }
    }
    for index in 0..n {
        if color[index] == WHITE {
            dfs_visit(&m, index, &mut color, &mut time, &mut d, &mut f);
        }
    }
    for index in 0..n {
        println!("{} {} {}", index + 1, d[index], f[index]);
    }
}
