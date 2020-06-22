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

const WHITE: u8 = 0;
const GRAY: u8 = 1;
const BLACK: u8 = 2;

const INFINITY: u64 = u64::max_value();

fn bfs(m: &Vec<Vec<u8>>, s: usize, color: &mut Vec<u8>, d: &mut Vec<u64>) {
    let mut q: VecDeque<u64> = VecDeque::new();
    q.push_back(s as u64);
    d[s] = 0;
    while q.len() > 0 {
        let u: usize = q.pop_front().unwrap() as usize;
        for v in 0..color.len() {
            if m[u][v] == 0 {
                continue;
            }
            if d[v] != INFINITY {
                continue;
            }
            d[v] = d[u] + 1;
            q.push_back(v as u64);
        }
    }
}

fn main() {
    let n: usize = read();
    let mut color: Vec<u8> = vec![WHITE; n];
    let mut m: Vec<Vec<u8>> = vec![vec![0; n]; n];
    let mut d: Vec<u64> = vec![INFINITY; n];
    for _ in 0..n {
        let index: usize = read();
        let count: usize = read();
        for _ in 0..count {
            let number: usize = read();
            m[index - 1][number - 1] = 1;
        }
    }
    bfs(&m, 0, &mut color, &mut d)
}
