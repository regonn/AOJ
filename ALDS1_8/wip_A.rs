use std::io::*;
use std::str::FromStr;

type NodeId = usize;

struct Node {
    parent: Option<NodeId>,
    left: Option<NodeId>,
    right: Option<NodeId>,
    value: i64,
}

struct Tree {
    nodes: Vec<Node>,
}

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

    let mut tree: Tree = vec![];
}
