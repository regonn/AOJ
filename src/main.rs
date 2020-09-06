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
    let mut w: f64 = read();

    let mut vw: Vec<Vec<f64>> = vec![vec![0.0; 2]; n as usize];

    for i in 0..n {
        let current_v: f64 = read();
        let current_w: f64 = read();
        vw[i as usize][0] = current_v;
        vw[i as usize][1] = current_w;
    }

    vw.sort_by(|a, b| (b[0] / b[1]).partial_cmp(&(a[0] / a[1])).unwrap());

    let mut ans: f64 = 0.0;

    for i in 0..(n as usize) {
        if w >= vw[i][1] {
            w -= vw[i][1];
            ans += vw[i][0];
        } else {
            ans += vw[i][0] / vw[i][1] * w;
            w = 0.0;
        }
    }

    println!("{}", ans);
}
