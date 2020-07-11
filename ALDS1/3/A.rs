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
    let mut stack: Vec<String> = vec![];
    let mut done: bool = false;
    while !done {
        let operation: String = read();
        if operation == "" {
            done = true;
        } else {
            match &*operation {
                "+" => {
                    let number_1: String = stack.pop().unwrap();
                    let number_2: String = stack.pop().unwrap();
                    let number_1_int: i64 = number_1.parse().unwrap();
                    let number_2_int: i64 = number_2.parse().unwrap();
                    let number: i64 = number_2_int + number_1_int;
                    stack.push(number.to_string());
                }
                "-" => {
                    let number_1: String = stack.pop().unwrap();
                    let number_2: String = stack.pop().unwrap();
                    let number_1_int: i64 = number_1.parse().unwrap();
                    let number_2_int: i64 = number_2.parse().unwrap();
                    let number: i64 = number_2_int - number_1_int;
                    stack.push(number.to_string());
                }
                "*" => {
                    let number_1: String = stack.pop().unwrap();
                    let number_2: String = stack.pop().unwrap();
                    let number_1_int: i64 = number_1.parse().unwrap();
                    let number_2_int: i64 = number_2.parse().unwrap();
                    let number: i64 = number_2_int * number_1_int;

                    stack.push(number.to_string());
                }
                _ => stack.push(operation),
            }
        }
    }
    println!("{}", stack[0]);
}
