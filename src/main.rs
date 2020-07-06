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

fn dfs(color: &mut Vec<Option<usize>>, target: usize, id: usize, g: &mut Vec<Vec<usize>>) {
    let mut s: VecDeque<usize> = VecDeque::new();
    s.push_back(target);
    color[target] = Some(id);

    while s.len() > 0 {
        let u: usize = s.pop_front().unwrap() as usize;
        for i in 0..g[u].len() {
            let v: usize = g[u][i];
            if color[v] == None {
                dfs(color, v, id, g);
            }
        }
    }
}

fn assign_color(color: &mut Vec<u32>, g: &mut Vec<Vec<u32>>, n: usize) {}

const WHITE: u32 = 0;
const GRAY: u32 = 1;
const BLACK: u32 = 2;
const INFINITY: u32 = 1000000;

fn main() {
    let n: usize = read();
    let mut g: Vec<Vec<u32>> = vec![vec![]; n];
    let mut color: Vec<u32> = vec![WHITE; n];
    for index_i in 0..n {
        for index_j in 0..n {
            let number: i32 = read();
            if number == -1 {
                g[index_i][index_j] = INFINITY;
            } else {
                g[index_i][index_j] = number as u32;
            }
        }
    }
    assign_color(&mut color, &mut g, n);
    let question_count: usize = read();
    for _ in 0..question_count {
        let from: usize = read();
        let to: usize = read();
        if color[from] == color[to] {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
