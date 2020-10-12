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

struct Element {
    prev: usize,
    next: usize,
    value: i64,
}

fn main() {
    let q: usize = read();
    let mut a: Vec<Element> = vec![];
    let mut index: usize = 0;
    a.push(Element {
        prev: 0,
        next: 0,
        value: 0,
    });

    for _ in 0..q {
        let query: i32 = read();
        match query {
            0 => {
                let number: i64 = read();
                let prev_index: usize = a[index].prev;
                let next_index: usize = index;
                a.push(Element {
                    prev: prev_index,
                    next: next_index,
                    value: number,
                });
                index = (a.len() - 1) as usize;
                a[prev_index].next = index;
                a[next_index].prev = index;
            }
            1 => {
                let move_index: i64 = read();
                if move_index > 0 {
                    for _ in 0..move_index {
                        index = a[index].next;
                    }
                } else {
                    for _ in 0..(-move_index) {
                        index = a[index].prev;
                    }
                }
            }
            2 => {
                let prev_index = a[index].prev;
                let next_index = a[index].next;
                a[prev_index].next = next_index;
                a[next_index].prev = prev_index;
                index = next_index;
            }
            _ => (),
        }
    }

    index = a[0].next;
    while index != 0 {
        println!("{}", a[index].value);
        index = a[index].next;
    }
}
