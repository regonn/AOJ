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
    let mut v: Vec<u64> = (0..n)
        .map(|_| {
            let number: u64 = read();
            number
        })
        .collect();
    let mut count: u64 = 0;
    let mut m: usize = 0;
    let mut g: Vec<usize> = vec![];
    let mut g_number = n;
    while g_number >= 1 {
        g_number /= 2;
        g.push(g_number);
        m += 1;
    }
    println!("{}", m);
    for (index, number) in g.iter().enumerate() {
        if index != (m - 1) {
            print!("{} ", number);
        } else {
            println!("{}", number);
        }
    }
    for index in 0..m {
        let g = g[index];
        for index_i in g..n {
            let v_value: u64 = v[index_i];
            let mut j: i64 = (index_i - g) as i64;
            while j >= 0 && v[j as usize] > v_value {
                v[(j as usize) + g] = v[j as usize];
                j -= g as i64;
                count += 1;
            }
            v[(j + (g as i64)) as usize] = v_value;
        }
    }

    println!("{}", count);
    for number in v {
        println!("{}", number);
    }
}
