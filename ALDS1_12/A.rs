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

const WHITE: u32 = 0;
const GRAY: u32 = 1;
const BLACK: u32 = 2;
const INFINITY: u32 = 1000000;
const MAX: usize = 100;

fn prim(m: &mut Vec<Vec<u32>>, n: usize) -> u32 {
    let mut d: Vec<u32> = vec![0; MAX];
    let mut p: Vec<i32> = vec![0; MAX];
    let mut color: Vec<u32> = vec![0; MAX];
    for index in 0..n {
        d[index] = INFINITY;
        p[index] = -1;
        color[index] = WHITE;
    }

    d[0] = 0;

    loop {
        let mut minv = INFINITY;
        let mut u: i32 = -1;
        for index in 0..n {
            if minv > d[index] && color[index] != BLACK {
                u = index as i32;
                minv = d[index];
            }
        }
        if u == -1 {
            break;
        }
        color[u as usize] = BLACK;
        for v in 0..n {
            if color[v] != BLACK && m[u as usize][v] != INFINITY {
                if d[v] > m[u as usize][v] {
                    d[v] = m[u as usize][v];
                    p[v] = u;
                    color[v] = GRAY;
                }
            }
        }
    }
    let mut sum = 0 as u32;
    for index in 0..n {
        if p[index] != -1 {
            sum += m[index][p[index] as usize]
        }
    }
    return sum;
}

fn main() {
    let n: usize = read();
    let mut m: Vec<Vec<u32>> = vec![vec![0; MAX]; MAX];
    for index_i in 0..n {
        for index_j in 0..n {
            let number: i32 = read();
            if number == -1 {
                m[index_i][index_j] = INFINITY;
            } else {
                m[index_i][index_j] = number as u32;
            }
        }
    }
    println!("{}", prim(&mut m, n));
}
