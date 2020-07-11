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

fn neighbor_indexes(dice_index: usize) -> Vec<usize> {
    match dice_index {
        0 => return vec![1, 2, 4, 3],
        1 => return vec![0, 3, 5, 2],
        2 => return vec![0, 1, 5, 4],
        3 => return vec![0, 4, 5, 1],
        4 => return vec![0, 2, 5, 3],
        5 => return vec![1, 3, 4, 2],
        _ => return vec![],
    }
}

fn right_index(dice_top: usize, dice_front: usize) -> usize {
    let neighbor_indexes: Vec<usize> = neighbor_indexes(dice_top);
    let position: usize = neighbor_indexes
        .iter()
        .position(|&i| i == dice_front)
        .unwrap();
    return neighbor_indexes[(position + 1) % 4];
}

fn main() {
    let mut dice: Vec<u8> = vec![];

    for _ in 0..6 {
        let number: u8 = read();
        dice.push(number);
    }

    let q: u8 = read();
    for _ in 0..q {
        let top_number: u8 = read();
        let front_number: u8 = read();

        let top_index: usize = dice.iter().position(|&n| n == top_number).unwrap();
        let front_index: usize = dice.iter().position(|&n| n == front_number).unwrap();
        println!("{}", dice[right_index(top_index, front_index)]);
    }
}
