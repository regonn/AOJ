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

fn print_board(x: &mut Vec<Vec<bool>>, row: &mut Vec<i8>) {
    for i in 0..N {
        for j in 0..N {
            if x[i][j] {
                if row[i] != j as i8 {
                    return;
                }
            }
        }
    }

    for i in 0..N {
        for j in 0..N {
            if row[i] == j as i8 {
                print!("Q");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn recursive(
    i: usize,
    x: &mut Vec<Vec<bool>>,
    row: &mut Vec<i8>,
    col: &mut Vec<i8>,
    dpos: &mut Vec<i8>,
    dneg: &mut Vec<i8>,
) {
    if i == N {
        print_board(x, row);
        return;
    }

    for j in 0..N {
        let i_j: usize = i + j;
        let i_j_n: usize = i + N - j - 1;

        if col[j] == NOT_FREE || dpos[i_j] == NOT_FREE || dneg[i_j_n] == NOT_FREE {
            continue;
        }
        row[i] = j as i8;
        col[j] = NOT_FREE;
        dpos[i_j] = NOT_FREE;
        dneg[i_j_n] = NOT_FREE;
        recursive(i + 1, x, row, col, dpos, dneg);
        row[i] = FREE;
        col[j] = FREE;
        dpos[i_j] = FREE;
        dneg[i_j_n] = FREE;
    }
}

fn main() {
    let mut row: Vec<i8> = vec![FREE; N];
    let mut col: Vec<i8> = vec![FREE; N];
    let mut dpos: Vec<i8> = vec![FREE; 2 * N - 1];
    let mut dneg: Vec<i8> = vec![FREE; 2 * N - 1];
    let mut x: Vec<Vec<bool>> = vec![vec![false; N]; N];

    let k: usize = read();

    for _ in 0..k {
        let r: usize = read();
        let c: usize = read();
        x[r][c] = true;
    }

    recursive(0, &mut x, &mut row, &mut col, &mut dpos, &mut dneg);
}
