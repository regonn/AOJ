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
        c.to_ascii_uppercase()
    }
}

fn convert(string: &str) -> String {
    string.chars().map(capslock).collect()
}

fn main() {
    let s = getline();

    println!("{}", convert(s.trim()));
}
