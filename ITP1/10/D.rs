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

fn cal_distance(n: usize, p: u8, x: Vec<f64>, y: Vec<f64>) -> (f64, Vec<f64>, Vec<f64>) {
    let mut distance: f64 = 0.0;

    if p == 0 {
        for number in 0..n {
            let max: f64 = (x[number] - y[number]).abs();
            if distance < max {
                distance = max;
            }
        }
    } else {
        for number in 0..n {
            distance += (x[number] - y[number]).abs().powf(p as f64);
        }
    }

    if p == 2 {
        return (distance.sqrt(), x, y);
    } else if p == 3 {
        return (distance.cbrt(), x, y);
    } else {
        return (distance, x, y);
    }
}

fn main() {
    let n: usize = read();
    let mut x: Vec<f64> = vec![];
    let mut y: Vec<f64> = vec![];

    for _ in 0..n {
        let number: f64 = read();
        x.push(number);
    }

    for _ in 0..n {
        let number: f64 = read();
        y.push(number);
    }

    let (cal_distance_1, x, y) = cal_distance(n, 1, x, y);
    let (cal_distance_2, x, y) = cal_distance(n, 2, x, y);
    let (cal_distance_3, x, y) = cal_distance(n, 3, x, y);
    let (cal_distance_infinity, x, y) = cal_distance(n, 0, x, y);

    println!("{}", cal_distance_1);
    println!("{}", cal_distance_2);
    println!("{}", cal_distance_3);
    println!("{}", cal_distance_infinity);
}
