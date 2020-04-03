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

fn update_dice_index(
    dice_top: usize,
    dice_front: usize,
    dice_right: usize,
    direct: char,
) -> (usize, usize, usize) {
    match direct {
        'N' => return (dice_front, reverse_index(dice_top), dice_right),
        'W' => return (dice_right, dice_front, reverse_index(dice_top)),
        'S' => return (reverse_index(dice_front), dice_top, dice_right),
        'E' => return (reverse_index(dice_right), dice_front, dice_top),
        _ => return (0, 0, 0),
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

fn dice_indexes(top_index: usize, front_index: usize) -> Vec<usize> {
    return vec![
        top_index,
        front_index,
        right_index(top_index, front_index),
        reverse_index(right_index(top_index, front_index)),
        reverse_index(front_index),
        reverse_index(top_index),
    ];
}

fn change_top_from(from_index: usize) -> (usize, usize) {
    let mut last_dice_top: usize = 0;
    let mut last_dice_front: usize = 1;
    match from_index {
        1 => {
            let (dice_top, dice_front, _) = update_dice_index(0, 1, 2, 'N');
            last_dice_top = dice_top;
            last_dice_front = dice_front;
        }
        2 => {
            let (dice_top, dice_front, _) = update_dice_index(0, 1, 2, 'W');
            last_dice_top = dice_top;
            last_dice_front = dice_front;
        }
        3 => {
            let (dice_top, dice_front, _) = update_dice_index(0, 1, 2, 'E');
            last_dice_top = dice_top;
            last_dice_front = dice_front;
        }
        4 => {
            let (dice_top, dice_front, _) = update_dice_index(0, 1, 2, 'S');
            last_dice_top = dice_top;
            last_dice_front = dice_front;
        }
        5 => {
            let (dice_top, dice_front, dice_right) = update_dice_index(0, 1, 2, 'N');
            let (dice_top, dice_front, _) =
                update_dice_index(dice_top, dice_front, dice_right, 'N');
            last_dice_top = dice_top;
            last_dice_front = dice_front;
        }
        _ => {}
    }
    return (last_dice_top, last_dice_front);
}

fn turn_positive(turn_count: u8, top_index: usize, front_index: usize) -> (usize, usize) {
    match (turn_count) % 4 {
        1 => {
            return (
                top_index,
                reverse_index(right_index(top_index, front_index)),
            )
        }
        2 => return (top_index, reverse_index(front_index)),
        3 => return (top_index, right_index(top_index, front_index)),
        _ => return (top_index, front_index),
    }
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

fn reverse_index(dice_index: usize) -> usize {
    match dice_index {
        0 => return 5,
        1 => return 4,
        2 => return 3,
        3 => return 2,
        4 => return 1,
        5 => return 0,
        _ => return 0,
    }
}

fn main() {
    let mut dice1: Vec<u8> = vec![];
    let mut dice2: Vec<u8> = vec![];
    let mut same_flag: bool = false;

    for _ in 0..6 {
        let number: u8 = read();
        dice1.push(number);
    }

    for _ in 0..6 {
        let number: u8 = read();
        dice2.push(number);
    }

    for top_index in 0..6 {
        let (dice_top, dice_front) = change_top_from(top_index);
        for turn_count in 0..4 {
            let (turned_dice_top, turned_dice_front) =
                turn_positive(turn_count, dice_top, dice_front);
            let turned_dice_indexes = dice_indexes(turned_dice_top, turned_dice_front);
            let mut compare_flag = true;
            for compare_index in 0..6 {
                if dice1[turned_dice_indexes[compare_index]] != dice2[compare_index] {
                    compare_flag = false;
                }
            }
            if compare_flag {
                same_flag = true;
            }
        }
    }

    if same_flag {
        println!("Yes")
    } else {
        println!("No")
    }
}
