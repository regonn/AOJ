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

fn assign_color(color: &mut Vec<Option<usize>>, g: &mut Vec<Vec<usize>>, n: usize) {
    let mut id: usize = 1;
    for u in 0..n {
        if color[u] == None {
            id = id + 1;
            dfs(color, u, id, g);
        }
    }
}

fn main() {
    let n: usize = read();
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    let mut color: Vec<Option<usize>> = vec![None; n];
    for _ in 0..m {
        let person_index1: usize = read();
        let person_index2: usize = read();
        g[person_index1].push(person_index2);
        g[person_index2].push(person_index1);
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
