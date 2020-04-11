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
    let v: Vec<(char, i32, usize)> = (0..n)
        .map(|i| {
            let s: String = read();
            let mut it = s.chars();
            let c = it.next().unwrap();
            let k: i32 = (it.next().unwrap() as i32) - 48;
            (c, k, i)
        })
        .collect();

    let mut bubble_sorted = v.clone();
    let mut bubble_flag: bool = true;
    while bubble_flag {
        bubble_flag = false;
        for index in (1..n).rev() {
            if bubble_sorted[index].1 < bubble_sorted[index - 1].1 {
                bubble_flag = true;
                bubble_sorted.swap(index, index - 1);
            }
        }
    }

    for (index, card) in bubble_sorted.iter().enumerate() {
        if index != n - 1 {
            print!("{}{} ", card.0, card.1);
        } else {
            println!("{}{}", card.0, card.1);
        }
    }

    let mut is_bubble_sorted_stable = true;
    for index_i in 0..n {
        for index_j in (index_i + 1)..n {
            if bubble_sorted[index_i].1 == bubble_sorted[index_j].1
                && bubble_sorted[index_i].2 > bubble_sorted[index_j].2
            {
                is_bubble_sorted_stable = false;
            }
        }
    }

    if is_bubble_sorted_stable {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    let mut select_sorted = v.clone();
    for index in 0..n {
        let mut min_j = index;
        for j in index..n {
            if select_sorted[j].1 < select_sorted[min_j].1 {
                min_j = j
            }
        }
        if index != min_j {
            select_sorted.swap(index, min_j);
        }
    }

    for (index, card) in select_sorted.iter().enumerate() {
        if index != n - 1 {
            print!("{}{} ", card.0, card.1);
        } else {
            println!("{}{}", card.0, card.1);
        }
    }

    let mut is_select_sorted_stable = true;
    for index_i in 0..n {
        for index_j in (index_i + 1)..n {
            if select_sorted[index_i].1 == select_sorted[index_j].1
                && select_sorted[index_i].2 > select_sorted[index_j].2
            {
                is_select_sorted_stable = false;
            }
        }
    }

    if is_select_sorted_stable {
        println!("Stable");
    } else {
        println!("Not stable");
    }
}
