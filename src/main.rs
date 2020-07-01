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

fn dfs(friends: &Vec<Vec<bool>>, n: usize) -> Vec<Vec<bool>> {
    let mut s: VecDeque<u64> = VecDeque::new();
    let mut connected_friends: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for index in 0..n {
        q.push_back(index as u64);
        while q.len() > 0 {
            let u: usize = q.pop_front().unwrap() as usize;
            for v in 0..n {
                if !friends[u][v] {
                    continue;
                }
                if connected_friends[u][v] && connected_friends[v][u] {
                    continue;
                }

                connected_friends[u][v] = true;
                connected_friends[v][u] = true;

                q.push_back(v as u64);
            }
        }
    }

    for index_i in 0..n {
        for index_j in 0..n {
            let mut number: usize = 0;
            if connected_friends[index_i][index_j] {
                number = 1;
            }
            print!("{} ", number);
        }
        println!();
    }

    return connected_friends;
}

fn assign_color(color: &mut Vec<Option<usize>>, g: &mut Vec<Vec<Option<usize>>>) {}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut g: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];
    let mut color: Vec<Option<usize>> = vec![None; n];
    for _ in 0..m {
        let person_index1: usize = read();
        let person_index2: usize = read();
        g[person_index1].push(Some(person_index2));
        g[person_index2].push(Some(person_index1));
    }
    assign_color(&mut color, &mut g);
    let question_count: usize = read();
    let connected_friends: Vec<Vec<bool>> = bfs(&friends, n);
    for _ in 0..question_count {
        let from: usize = read();
        let to: usize = read();
        if connected_friends[from][to] {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
