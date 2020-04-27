use std::io::*;
use std::str::FromStr;

type Card = (char, u32, usize);

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

fn partition(a: &mut Vec<Card>, p: usize, r: usize) -> usize {
    let mut i: i32 = (p as i32) - 1;
    let x: Card = a[r];

    for j in p..r {
        if a[j].1 <= x.1 {
            i += 1;
            a.swap(i as usize, j);
        }
    }
    a.swap((i+1) as usize, r);
    return (i + 1) as usize;
}

fn quicksort(a: &mut Vec<Card>, p: usize, r: usize) {
    if p < r {
        let q: usize = partition(a, p, r);
        if q > 0{
            quicksort(a, p, q-1);
        }
        quicksort(a, q+1, r);
    }
}

fn check_stable(a: &Vec<Card>)-> bool {
    let n = a.len();
    let mut is_bubble_sorted_stable: bool = true;
    for index_i in 0..(n - 2) {
        let mut index_j = index_i + 1;
        while a[index_i].1 == a[index_j].1 && index_j < (n - 1) {
            if a[index_i].2 > a[index_j].2 {
                is_bubble_sorted_stable = false;
            }
            index_j += 1;
        }
    }
    return is_bubble_sorted_stable;
}

fn main() {
    let n: usize = read();
    let mut a: Vec<Card> = (0..n)
        .map(|index| {
            let suit_str: String = read();
            let suit_char: char = suit_str.chars().next().unwrap();
            let number: u32 = read();
            (suit_char, number, index)
        })
        .collect();
    quicksort(&mut a, 0, n-1);

    if check_stable(&a){
        println!("Stable");
    }else{
        println!("Not stable");
    }

    for card in a.clone() {
        println!("{} {}", card.0, card.1);
    }
}
