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

fn main() {
    let mut residences: [[[i8; 10]; 3]; 4] = [[[0; 10]; 3]; 4];

    let n: u8 = read();
    for _ in 0..n {
        let b: usize = read();
        let f: usize = read();
        let r: usize = read();
        let v: i8 = read();
        residences[b - 1][f - 1][r - 1] += v;
    }

    let mut building_index: u8 = 0;
    for building in residences.iter() {
        for floor in building.iter() {
            for room in floor.iter() {
                print!(" {}", room);
            }
            println!();
        }
        building_index += 1;
        if building_index != 4 {
            println!("####################")
        }
    }
}
