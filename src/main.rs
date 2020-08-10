use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
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

const N: usize = 4;
const N2: usize = 16;

const DX: [i32; 4] = [0, -1, 0, 1];
const DY: [i32; 4] = [1, 0, -1, 0];
const DIR: [&str; 4] = [&"r", &"u", &"l", &"d"];

#[derive(PartialEq, Eq, Hash, Clone)]
struct Puzzle {
    f: Vec<usize>,
    space: usize,
    md: usize,
}

#[derive(Eq)]
struct State {
    puzzle: Puzzle,
    estimated: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.estimated.cmp(&other.estimated) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match Some(self.cmp(other)) {
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.estimated == other.estimated
    }
}

fn puzzle_key(puzzle: Puzzle) -> String {
    return puzzle
        .f
        .iter()
        .map(|&s| s.to_string() + ",")
        .collect::<String>();
}

fn get_all_md(puzzle: Puzzle, mdt: Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    for i in 0..N2 {
        if puzzle.f[i] == N2 {
            continue;
        }
        sum += mdt[i][puzzle.f[i] - 1];
    }
    return sum;
}

fn dfs(depth: i32, prev: i32, puzzle: &mut Puzzle) -> bool {
    return false;
}

fn iterative_deepening(puzzle: Puzzle, mdt: Vec<Vec<usize>>) -> String {
    let mut new_puzzle = puzzle.clone();
    new_puzzle.md = get_all_md(puzzle.clone(), mdt);
    let mut path: Vec<usize> = vec![];

    for limit in new_puzzle.md..100 {
        if dfs(0, -100, &mut new_puzzle) {
            let mut answer: String = String::new();
            for index in 0..limit {
                answer += DIR[path[index]];
            }
        }
    }

    return "".to_string();
}

fn main() {
    let mut init_numbers: Vec<usize> = vec![0; N2];
    let mut mdt: Vec<Vec<usize>> = vec![vec![0; N2]; N2];
    let mut init_space: usize = 0;
    for i in 0..N2 {
        for j in 0..N2 {
            mdt[i][j] = ((i / N) as i32 - (j / N) as i32).abs() as usize
                + ((i % N) as i32 - (j % N) as i32).abs() as usize;
        }
    }
    for i in 0..N2 {
        let number: usize = read();
        init_numbers[i] = number;

        if init_numbers[i] == 0 {
            init_numbers[i] = N2;
            init_space = i;
        }
    }

    let puzzle = Puzzle {
        f: init_numbers,
        space: init_space,
        md: 0,
    };

    let ans: String = iterative_deepening(puzzle, mdt);
    println!("{}", ans.len());
}
