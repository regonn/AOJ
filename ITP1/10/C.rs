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
    let mut done: bool = false;
    while !done {
        let n: u16 = read();
        if n == 0 {
            done = true;
        } else {
            let mut scores: Vec<f32> = vec![];
            for _ in 0..n {
                let score: f32 = read();
                scores.push(score)
            }
            let scores_sum: f32 = scores.iter().fold(0.0, |sum, score| sum + score);
            let scores_ave: f32 = scores_sum / (scores.len() as f32);
            let sd_2: f32 = scores
                .iter()
                .fold(0.0, |sum, score| sum + (score - scores_ave).powf(2.0))
                / (scores.len() as f32);

            println!("{}", sd_2.sqrt());
        }
    }
}
