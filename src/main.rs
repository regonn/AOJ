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
    let mut st: Vec<(u32, u32)> = vec![(0, 0); n];
    let mut time = 0;
    let mut answer = 0;

    for i in 0..n {
        let s: u32 = read();
        let t: u32 = read();
        st[i] = (s, t)
    }

    st.sort_by_key(|key| (key.1, key.0));

    for &(s, t) in &st {
        if s < time {
            continue;
        }
        time = t + 1;
        answer += 1;
    }

    println!("{}", answer);
}
