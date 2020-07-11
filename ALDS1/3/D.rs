use std::collections::VecDeque;
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
    let mut stack1 = VecDeque::new();
    let mut stack2 = VecDeque::new();
    let mut area: u32 = 0;
    let data: String = read();
    for (index, data_char) in data.chars().enumerate() {
        match data_char {
            '/' => {
                if !stack1.is_empty() {
                    let stacked_index: usize = stack1.pop_back().unwrap();
                    let this_area: u32 = (index - stacked_index) as u32;
                    area += this_area;
                    let mut done: bool = false;
                    let mut stacked2_area: u32 = 0;
                    while !done {
                        let stacked_index_2: (usize, u32, bool) = stack2.pop_back().unwrap();
                        if stacked_index_2.2 {
                            stacked2_area += stacked_index_2.1;
                        } else {
                            done = true;
                            stack2.push_back((stacked_index_2.0, stacked2_area + this_area, true))
                        }
                    }
                }
                ()
            }
            '\\' => {
                stack1.push_back(index);
                stack2.push_back((index, 0, false));
            }
            _ => (),
        }
    }
    println!("{}", area);
    let stack2_answer: Vec<(usize, u32, bool)> = stack2.iter().filter(|&x| x.2).cloned().collect();
    let stack2_answer_len = stack2_answer.len();

    if stack2_answer_len == 0 {
        println!("0")
    } else {
        print!("{} ", stack2_answer.len());

        for (index, stack) in stack2_answer.iter().enumerate() {
            if index != (stack2_answer.len() - 1) {
                print!("{} ", stack.1);
            } else {
                println!("{}", stack.1);
            }
        }
    }
}
