use std::ascii::AsciiExt;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    ret
}

fn capslock(c: char) -> char {
    if c.is_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c
    }
}

fn main() {
    let mut done: bool = false;
    let mut code_counts: Vec<u8> = vec![0; 26];
    while !done {
        let s = getline();

        if s == "" {
            done = true;
        } else {
            for c in s.chars().map(capslock) {
                let code = c as u8;
                if code >= b'a' && code <= b'z' {
                    code_counts[(code - b'a') as usize] += 1;
                }
            }
        }
    }
    for (index, code_count) in code_counts.iter().enumerate() {
        println!("{} : {}", ((index as u8) + b'a') as char, code_count);
    }
}
