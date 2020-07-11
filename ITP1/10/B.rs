use std::f64;
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
    let a: f64 = read();
    let b: f64 = read();
    let c: f64 = read();
    let c_radian: f64 = (c / 180.0) * f64::consts::PI;
    println!("{}", a * b * c_radian.sin() * 0.5);
    println!(
        "{}",
        a + b + (a.powf(2.0) + b.powf(2.0) - 2.0 * a * b * c_radian.cos()).sqrt()
    );
    println!("{}", b * c_radian.sin());
}
