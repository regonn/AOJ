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

fn main() {
    let n: usize = read();
    let mut s: usize = 10_000;
    let a: Vec<usize> = (0..n)
        .map(|_| {
            let number: usize = read();
            s = min(s, number);
            number
        })
        .collect();
    let mut answer: isize = 0;
    let mut b = a.clone();
    let mut v: Vec<bool> = vec![false; n];
    let mut t: Vec<usize> = vec![0; 10_001];

    b.sort();
    for index in 0..n {
        t[b[index]] = index;
    }

    for index in 0..n {
        if v[index] {
            continue;
        }

        let mut cur: usize = index;
        let mut S: isize = 0;
        let mut m: isize = 10_000;
        let mut an: isize = 0;
        loop {
            v[cur] = true;
            an += 1;
            let val = a[cur];
            m = min(m, val as isize);
            S += val as isize;
            cur = t[val];
            if v[cur] {
                break;
            }
        }
        answer += min(S + (an - 2) * m, m + S + (an + 1) * s as isize);
    }

    println!("{}", answer);
}
