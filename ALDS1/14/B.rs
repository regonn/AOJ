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

fn kmp_table(pattern: &Vec<u8>, m: usize, mut kmp: Vec<usize>) -> Vec<usize> {
    let mut len: usize = 0;
    kmp[0] = 0;
    let mut i: usize = 1;
    while i < m {
        if pattern[i] == pattern[len] {
            len = len + 1;
            kmp[i] = len;
            i = i + 1;
        } else {
            if len != 0 {
                len = kmp[len - 1];
            } else {
                kmp[i] = 0;
                i = i + 1;
            }
        }
    }
    kmp
}

fn kmp(string: &Vec<u8>, pattern: &Vec<u8>) -> () {
    let mut out: String = String::new();
    let n = string.len();
    let m = pattern.len();
    let kmp = kmp_table(&pattern, m, vec![0usize; m]);
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < n {
        if pattern[j] == string[i] {
            i += 1;
            j += 1;
        }
        if j == m {
            out.push_str(&(i - j).to_string());
            out.push('\n');
            j = kmp[j - 1];
        } else if i < n && pattern[j] != string[i] {
            if j != 0 {
                j = kmp[j - 1];
            } else {
                i += 1;
            }
        }
    }
    print!("{}", out);
}

fn main() {
    let t: String = read();
    let p: String = read();
    let string: Vec<u8> = t.as_bytes().to_vec();
    let pattern: Vec<u8> = p.as_bytes().to_vec();
    kmp(&string, &pattern);
}
