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

#[derive(PartialEq, Eq, Hash, Clone)]
struct Puzzle {
    f: Vec<usize>,
    space: usize,
    md: usize,
    cost: usize,
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

fn astar(puzzle: Puzzle, mdt: Vec<Vec<usize>>) -> usize {
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    let mut v = HashSet::new();
    let mut new_puzzle = puzzle.clone();
    let v_key: String = puzzle_key(new_puzzle.clone());
    new_puzzle.md = get_all_md(puzzle.clone(), mdt.clone());
    let initial: State = State {
        puzzle: new_puzzle,
        estimated: get_all_md(puzzle.clone(), mdt.clone()),
    };
    pq.push(initial);
    v.insert(v_key);

    while pq.len() > 0 {
        let st: State = pq.pop().unwrap();

        let u_puzzle = st.puzzle;

        if u_puzzle.md == 0 {
            return u_puzzle.cost;
        }

        let u_key: String = puzzle_key(u_puzzle.clone());
        v.insert(u_key);

        let sx = u_puzzle.space / N;
        let sy = u_puzzle.space % N;
        for r in 0..4 {
            let tx: i32 = sx as i32 + DX[r];
            let ty: i32 = sy as i32 + DY[r];

            if tx < 0 || ty < 0 || tx >= N as i32 || ty >= N as i32 {
                continue;
            }

            let mut v_puzzle = u_puzzle.clone();
            v_puzzle.md -=
                mdt[tx as usize * N + ty as usize][v_puzzle.f[tx as usize * N + ty as usize] - 1];
            v_puzzle.md +=
                mdt[sx as usize * N + sy as usize][v_puzzle.f[tx as usize * N + ty as usize] - 1];
            v_puzzle.f.swap(sx * N + sy, tx as usize * N + ty as usize);
            v_puzzle.space = tx as usize * N + ty as usize;
            let v_puzzle_key: String = puzzle_key(v_puzzle.clone());
            let mut v_puzzle_cost = v_puzzle.cost;
            if !v.contains(&v_puzzle_key) {
                let v_puzzle_md: usize = v_puzzle.md;
                v_puzzle_cost += 1;
                v_puzzle.cost += 1;
                let news: State = State {
                    puzzle: v_puzzle,
                    estimated: v_puzzle_cost + v_puzzle_md,
                };
                pq.push(news);
            }
        }
    }
    return 0;
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
        cost: 0,
    };

    let ans: usize = astar(puzzle, mdt);
    println!("{}", ans);
}
