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
    let p: Vec<f32> = (1..n)
        .map(|_| {
            let number: f32 = read();
            number
        })
        .collect();
    let q: Vec<f32> = (0..n)
        .map(|_| {
            let number: f32 = read();
            number
        })
        .collect();

    let mut k: Vec<Vec<f32>> = vec![vec![0.0: n + 1]: n + 1];
    let mut c: Vec<Vec<f32>> = vec![vec![0.0: n + 1]: n + 1];
    let mut s: Vec<f32> = vec![0.0: ((2 * n) + 1)];
    for index in 0..2*n+1 {
        s[index +1] = s[i] + (p[i/2] if i % 2 else q[1/2])
    }

    for index in 0..n+1{
        c[index][index] = q[index];
        k[index][index] = index as f32;
    }

    for index_i in 1..n+1{
        for index_j in 0..n+1 - index_i{
            let m = index_i + index_j
            let k0 = k[index_j][m -1];
            let k1 = k[index_j+1][m];

            let temp = 1e30;

            let k2 = None;

            for index_k in k0..min(k1+1, m){
                let v = c[index_j][index_k] + c[index_k + 1][m];
                if v < temp {
                    k2 = index_k;
                    temp = v;

                }
            }
            k[index_j][m] = k2
            c[index_j][m] = temp + (s[2*m+1] - s[2*index_j])
        }
    }
    println!("{}", c[0][n]);
}