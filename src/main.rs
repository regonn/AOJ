use std::collections::HashMap;
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

const N: usize = 3;
const N2: usize = 9;

const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, -1, 0, 1];
const DIR: [char; 4] = ['u', 'l', 'd', 'r'];

#[derive(PartialEq, Eq, Hash, Clone)]
struct Puzzle {
    f: Vec<usize>,
    space: usize,
    path: Option<String>,
}

fn is_target(puzzle: Puzzle) -> bool {
    for index in 0..N2 {
        if puzzle.f[index] != (index + 1) {
            return false;
        }
    }
    return true;
}

fn puzzle_key(puzzle: Puzzle) -> String {
    return puzzle.f.iter().map(|&s| s.to_string()).collect::<String>();
}

fn bfs(puzzle: Puzzle) -> String {
    let mut q: VecDeque<Puzzle> = VecDeque::new();
    let mut v = HashMap::new();
    let mut new_puzzle = puzzle;
    new_puzzle.path = Some(("").to_string());
    let v_key: String = puzzle_key(new_puzzle.clone());
    q.push_back(new_puzzle);
    v.insert(v_key, true);
    while q.len() > 0 {
        let u_puzzle: Puzzle = q.pop_front().unwrap();
        if is_target(u_puzzle.clone()) {
            return u_puzzle.path.as_ref().unwrap().to_string();
        }
        let sx = u_puzzle.space / N;
        let sy = u_puzzle.space % N;
        for r in 0..4 {
            let tx: i32 = sx as i32 + DX[r];
            let ty: i32 = sy as i32 + DY[r];
            if tx < 0 || ty < 0 || tx >= N as i32 || ty >= N as i32 {
                continue;
            }
            let mut v_puzzle = u_puzzle.clone();
            let temp = v_puzzle.f[u_puzzle.space];
            v_puzzle.f[u_puzzle.space] = v_puzzle.f[tx as usize * N + ty as usize];
            v_puzzle.f[tx as usize * N + ty as usize] = temp;
            v_puzzle.space = tx as usize * N + ty as usize;
            let v_puzzle_key: String = puzzle_key(v_puzzle.clone());
            let v_bool: &bool = v.get(&v_puzzle_key).unwrap();
            if !*v_bool {
                v.insert(v_puzzle_key, true);
            }

            // v_puzzle
            //     .f
            //     .swap(u_puzzle.space, tx as usize * N + ty as usize)
        }
    }
    return "".to_string();
}

fn main() {
    let mut init_numbers: Vec<usize> = vec![0; N2];
    let mut init_space: usize = 0;
    for i in 0..N2 {
        let number: usize = read();
        init_numbers[i] = number;

        if init_numbers[i] == 0 {
            init_numbers[i] = N2;
            init_space = i;
        }
    }

    let mut puzzle = Puzzle {
        f: init_numbers,
        space: init_space,
        path: None,
    };

    let ans: String = bfs(puzzle);
    println!("{}", ans);
}
