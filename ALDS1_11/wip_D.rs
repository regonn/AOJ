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

const INFINITY: u64 = 10000000;

fn bfs(m: &Vec<Vec<bool>>, n: usize, from: usize, to: usize) -> bool {
    let mut d: Vec<u64> = vec![INFINITY; n];
    let mut q: VecDeque<u64> = VecDeque::new();
    q.push_back(from as u64);
    d[from] = 0;
    while q.len() > 0 {
        let u: usize = q.pop_front().unwrap() as usize;
        for v in 0..n {
            if !m[u][v] {
                continue;
            }
            if d[v] != INFINITY {
                continue;
            }
            d[v] = d[u] + 1;
            q.push_back(v as u64);
        }
    }
    
    for index in 0..d.len() {
        let number: i64;
        if d[index] == INFINITY {
            number = -1;
        } else {
            number = d[index] as i64;
        }
        println!("{} {}", index + 1, number);
    }
}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut friends: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for _ in 0..m {
        let person_index1: usize = read();
        let person_index2: usize = read();
        friends[person_index1][person_index2] = true;
        friends[person_index2][person_index1] = true;
    }
    let question_count: usize = read();
    for _ in 0..question_count {
        let from: usize = read();
        let to: usize = read();
        if bfs(friends, n, from, to) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
