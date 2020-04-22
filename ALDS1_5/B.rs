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

fn merge(a: &mut Vec<u64>, left: usize, mid: usize, right: usize) -> usize {
    let n1: usize = mid - left;
    let n2: usize = right - mid;
    let mut l: Vec<u64> = (0..n1)
        .map(|i| {
            let number: u64 = a[left + i];
            number
        })
        .collect();
    let mut r: Vec<u64> = (0..n2)
        .map(|i| {
            let number: u64 = a[mid + i];
            number
        })
        .collect();
    l.push(u64::max_value());
    r.push(u64::max_value());
    let mut i: usize = 0;
    let mut j: usize = 0;
    for k in left..right {
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];
            j += 1;
        }
    }
    return i + j;
}

fn merge_sort(a: &mut Vec<u64>, left: usize, right: usize) -> usize {
    let mut count: usize = 0;

    if left + 1 < right {
        let mid: usize = (left + right) / 2;
        count += merge_sort(a, left, mid);
        count += merge_sort(a, mid, right);
        count += merge(a, left, mid, right);
    }
    return count;
}

fn main() {
    let n: usize = read();
    let mut a: Vec<u64> = (0..n)
        .map(|_| {
            let number: u64 = read();
            number
        })
        .collect();
    let count_number = merge_sort(&mut a, 0, n);
    for (index, number) in a.iter().enumerate() {
        if (index + 1) != a.len() {
            print!("{} ", number);
        } else {
            println!("{}", number);
        }
    }
    println!("{}", count_number);
}
