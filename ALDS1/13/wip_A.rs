use std::collections::VecDeque;
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

const N: usize = 8;
const FREE: i8 = -1;
const NOT_FREE: i8 = 1;

fn printBoard(x: Vec<Vec<bool>>, row: Vec<i8>) {}

fn recursive(
    i: usize,
    x: &mut Vec<Vec<bool>>,
    row: &mut Vec<i8>,
    col: &mut Vec<i8>,
    dpos: &mut Vec<i8>,
    dneg: &mut Vec<i8>,
) {
    if i == N {
        printBoard(*x, *row);
        return;
    }

    for j in 0..N {
        if col[j] == NOT_FREE || dpos[i + j] == NOT_FREE || dneg[i - j + N - 1] == NOT_FREE {
            continue;
        }
        row[i] = j as i8;
        col[j] = NOT_FREE;
        dpos[i + j] = NOT_FREE;
        dneg[i - j + N - 1] = NOT_FREE;
        recursive(i + 1, x, row, col, dpos, dneg);
        row[i] = FREE;
        col[j] = FREE;
        dpos[i + j] = FREE;
        dneg[i - j + N - 1] = FREE;
    }
}

fn main() {
    let mut row: Vec<i8> = vec![FREE; N];
    let mut col: Vec<i8> = vec![FREE; N];
    let mut dpos: Vec<i8> = vec![FREE; N];
    let mut dneg: Vec<i8> = vec![FREE; N];
    let mut x: Vec<Vec<bool>> = vec![vec![false; N]; N];

    let k: usize = read();

    for _ in 0..k {
        let r: usize = read();
        let c: usize = read();
        x[r][c] = true;
    }

    recursive(0);
}
