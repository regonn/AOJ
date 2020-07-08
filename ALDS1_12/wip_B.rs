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

fn dijkstra(m: &mut Vec<Vec<u32>>, n: usize) {
    let mut d: Vec<u32> = vec![0; MAX];
    let mut color: Vec<u32> = vec![0; MAX];
    for index in 0..n {
        d[index] = INFINITY;
        color[index] = WHITE;
    }

    d[0] = 0;
    color[0] = GRAY;

    loop {
        let mut minv = INFINITY;
        let mut u: i32 = -1;
        for index in 0..n {
            if minv > d[index] && color[index] != BLACK {
                u = index as i32;
                minv = d[index]
            }
        }
        if u == -1 {
            break;
        }
        color[u as usize] = BLACK;
        for v in 0..n {
            if color[v] != BLACK && m[u as usize][v] != INFINITY {
                if d[v] > d[u as usize] + m[u as usize][v] {
                    d[v] = d[u as usize] + m[u as usize][v];
                    color[v] = GRAY
                }
            }
        }
    }
}

fn main() {
    let n: usize = read();
    let mut m: Vec<Vec<u32>> = vec![vec![INFINITY; MAX]; MAX];
    for index_n in 0..n {
        let u: usize = read();
        let k: usize = read();
        for index_k in 0..k {
            let v: usize = read();
            let c: u32 = read();
            m[u][v] = c;
        }
    }

    dijkstra(&mut m, n);
}
