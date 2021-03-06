use std::cmp::max;
use std::cmp::min;
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

fn get_all_md(puzzle: Puzzle, mdt: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    for i in 0..N2 {
        if puzzle.f[i] == N2 {
            continue;
        }
        sum += mdt[i][puzzle.f[i] - 1];
    }
    return sum;
}

fn dfs(
    depth: i32,
    prev: i32,
    puzzle: &mut Puzzle,
    limit: usize,
    mdt: &Vec<Vec<usize>>,
    path: &mut Vec<usize>,
) -> bool {
    if puzzle.md == 0 {
        return true;
    }
    if depth + puzzle.md as i32 > limit as i32 {
        return false;
    }
    let space_x = puzzle.space / N;
    let space_y = puzzle.space % N;

    for r in 0..4 {
        let target_x: i32 = space_x as i32 + DX[r];
        let target_y: i32 = space_y as i32 + DY[r];
        if target_x < 0 || target_y < 0 || target_x >= N as i32 || target_y >= N as i32 {
            continue;
        }
        if max(prev, r as i32) - min(prev, r as i32) == 2 {
            continue;
        }
        let tmp_puzzle = puzzle.clone();
        puzzle.md -= mdt[target_x as usize * N + target_y as usize]
            [puzzle.f[target_x as usize * N + target_y as usize] - 1];
        puzzle.md +=
            mdt[space_x * N + space_y][puzzle.f[target_x as usize * N + target_y as usize] - 1];
        puzzle.f.swap(
            target_x as usize * N + target_y as usize,
            space_x * N + space_y,
        );
        puzzle.space = target_x as usize * N + target_y as usize;
        if dfs(depth + 1, r as i32, puzzle, limit, mdt, path) {
            path[depth as usize] = r;
            return true;
        }

        puzzle.f = tmp_puzzle.f;
        puzzle.md = tmp_puzzle.md;
        puzzle.space = tmp_puzzle.space;
    }

    return false;
}

fn iterative_deepening(puzzle: Puzzle, mdt: &Vec<Vec<usize>>) -> String {
    let mut new_puzzle = puzzle.clone();
    new_puzzle.md = get_all_md(puzzle.clone(), mdt);
    let mut path: Vec<usize> = vec![0; 100];

    for limit in new_puzzle.md..100 {
        if dfs(0, -100, &mut new_puzzle, limit, mdt, &mut path) {
            let mut answer: String = String::new();
            for index in 0..limit {
                answer += DIR[path[index]];
            }
            return answer;
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

    let ans: String = iterative_deepening(puzzle, &mdt);
    println!("{}", ans.len());
}
