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

fn dijkstra(adj: &mut Vec<Vec<(usize, u32)>>, n: usize) {
    let mut pq: VecDeque<(i32, u32)> = VecDeque::new();
    let mut d: Vec<u32> = vec![INFINITY; MAX];
    let mut color: Vec<u32> = vec![WHITE; MAX];

    d[0] = 0;
    pq.push_back((0, 0));
    color[0] = GRAY;

    while pq.len() > 0 {
        let (first, u): (i32, u32) = pq.pop_front().unwrap();

        color[u as usize] = GRAY;

        if (d[u as usize] as i32) < first * -1 {
            continue;
        }

        for j in 0..adj[u as usize].len() {
            let (v, second) = adj[u as usize][j];
            if color[v] == BLACK {
                continue;
            }
            if d[v] > d[u as usize] + second {
                d[v] = d[u as usize] + second;
                pq.push_back(((d[v] as i32 * -1), v as u32));
                color[v as usize] = GRAY;
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
    let mut adj: Vec<Vec<(usize, u32)>> = vec![vec![]; MAX];
    for _ in 0..n {
        let u: usize = read();
        let k: usize = read();
        for _ in 0..k {
            let v: usize = read();
            let c: u32 = read();
            adj[u].push((v, c));
        }
    }

    dijkstra(&mut adj, n);
}
