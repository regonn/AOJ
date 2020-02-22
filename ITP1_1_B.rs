use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let x: i32 = iter.next().unwrap().parse().unwrap();
    println!("{}", x * x * x);
}