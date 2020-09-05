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
    let n: u32 = read();
    let w: u32 = read();
    let mut v_i: Vec<u32> = vec![0; n as usize];
    let mut w_i: Vec<u32> = vec![0; n as usize];
    let mut v_per_w_i: Vec<f32> = vec![0.0; n as usize];
    let mut knapsack_w: u32 = 0;
    let mut knapsack_v: f32 = 0.0;
    for i in 0..n {
        let current_v: u32 = read();
        let current_w: u32 = read();
        v_i[i as usize] = current_v;
        w_i[i as usize] = current_w;
        v_per_w_i[i as usize] = current_v as f32 / current_w as f32;
    }
    while knapsack_w < w {
        let mut value_i: Option<usize> = None;
        let mut value: f32 = 0.0;
        for i in 0..n {
            if w_i[i as usize] > 0 && value < v_per_w_i[i as usize] {
                value_i = Some(i as usize);
                value = v_per_w_i[i as usize];
            }
        }
        if value_i.is_none() {
            break;
        } else {
            let i = value_i.unwrap();
            let capacity = w - knapsack_w;
            if capacity > w_i[i] {
                knapsack_w += w_i[i];
                w_i[i] -= w_i[i];
                knapsack_v += v_i[i] as f32;
            } else {
                knapsack_w = w;
                knapsack_v += v_per_w_i[i] * capacity as f32;
            }
        }
    }
    println!("{}", knapsack_v);
}
