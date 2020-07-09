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

const WHITE: u32 = 0;
const GRAY: u32 = 1;
const BLACK: u32 = 2;
const INFINITY: u32 = 1000000;
const MAX: usize = 10000;

fn dijkstra(m: &mut Vec<(usize, u32)>, n: usize) {
    let mut pq: VecDeque<(usize, u32)> = VecDeque::new();
    let mut d: Vec<u32> = vec![0; MAX];
    let mut color: Vec<u32> = vec![0; MAX];

    d[0] = 0;
    pq.push_back((0, 0));
    color[0] = GRAY;

    while pq.len() > 0 {
        let (first, u): (usize, u32) = pq.pop_front().unwrap();

        color[u as usize] = GRAY;

        if d[u as usize] as i32 < (first as i32) * (-1) {
            continue
        }

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
            if color[v] != BLACK && adj[u as usize][v] != INFINITY {
                if d[v] > d[u as usize] + m[u as usize][v] {
                    d[v] = d[u as usize] + m[u as usize][v];
                    color[v] = GRAY
                }
            }
        }
    }
    for index in 0..n {
        print!("{} ", index);
        println!("{}", d[index]);
    }
}

fn main() {
    let n: usize = read();
    let mut adj: Vec<(usize, u32)> = vec![(0, INFINITY); MAX];
    for _ in 0..n {
        let u: usize = read();
        let k: usize = read();
        for _ in 0..k {
            let v: usize = read();
            let c: u32 = read();
            adj[u] = (v, c);
        }
    }

    dijkstra(&mut adj, n);
}
