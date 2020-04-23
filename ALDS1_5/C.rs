use std::io::*;
use std::str::FromStr;
use std::f64;

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

fn kock(n: u8, p1: (f64, f64), p2: (f64, f64)) {
    if n == 0 {
        return;
    }
    let s: (f64, f64) = ((2.0*p1.0+1.0*p2.0)/3.0,(2.0*p1.1+1.0*p2.1)/3.0);
    let t: (f64, f64) = ((1.0*p1.0+2.0*p2.0)/3.0,(1.0*p1.1+2.0*p2.1)/3.0);
    let u: (f64, f64) = ((t.0 -s.0)*((1.0/3.0) * f64::consts::PI).cos() - (t.1 -s.1)*((1.0/3.0) * f64::consts::PI).sin() + s.0, (t.0 -s.0)*((1.0/3.0) * f64::consts::PI).sin() + (t.1 - s.1)*((1.0/3.0) * f64::consts::PI).cos() + s.1);
    kock(n-1, p1, s);
    println!("{} {}", s.0, s.1);
    kock(n-1, s, u);
    println!("{} {}", u.0, u.1);
    kock(n-1, u, t);
    println!("{} {}", t.0, t.1);
    kock(n-1, t, p2);
}

fn main() {
    let n: u8 = read();
    println!("0.0 0.0");
    kock(n, (0.0,0.0), (100.0,0.0));
    println!("100.0 0.0");
}
