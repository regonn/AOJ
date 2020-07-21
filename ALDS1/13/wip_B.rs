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

#[derive(PartialEq, Eq, Hash)]
struct Puzzle {
    f: Vec<usize>,
    space: usize,
    path: Option<String>,
}

fn is_target(puzzle: &Puzzle) -> bool {
    for index in 0..N2 {
        if puzzle.f[index] != (index + 1) {
            return false;
        }
    }
    return true;
}

fn bfs(puzzle: &mut Puzzle) -> String {
    let mut q: VecDeque<&mut Puzzle> = VecDeque::new();
    let mut v = HashMap::new();
    puzzle.path = Some(("").to_string());
    let v_key: String = puzzle.f.iter().map(|&s| s.to_string()).collect::<String>();
    q.push_back(puzzle);
    v.insert(v_key, true);
    while q.len() > 0 {
        let u: &Puzzle = q.pop_front().unwrap();
        if is_target(u) {
            return u.path.as_ref().unwrap().to_string();
        }
        let sx = u.space / N;
        let sy = u.space % N;
        for r in 0..4 {}
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

    let ans: String = bfs(&mut puzzle);
    println!("{}", ans);
}
