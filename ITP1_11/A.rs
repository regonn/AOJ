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

fn main() {
    let mut dice: Vec<u8> = vec![];
    let mut last_dice_top: usize = 0;
    let mut last_dice_front: usize = 1;
    let mut last_dice_right: usize = 2;

    for _ in 0..6 {
        let number: u8 = read();
        dice.push(number);
    }

    let directs: String = read();
    for direct in directs.chars() {
        let (dice_top, dice_front, dice_right) =
            update_dice_index(last_dice_top, last_dice_front, last_dice_right, direct);
        last_dice_top = dice_top;
        last_dice_front = dice_front;
        last_dice_right = dice_right;
    }
    println!("{}", dice[last_dice_top]);
}
