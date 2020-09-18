use std::io::*;
use std::str::FromStr;

type NodeId = usize;

struct Node {
    id: NodeId,
    parent: Option<NodeId>,
    left: Option<NodeId>,
    right: Option<NodeId>,
    value: i64,
}

struct Tree {
    root: Option<NodeId>,
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            root: None,
            nodes: Vec::new(),
        }
    }
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
    let s: String = read();
    let s_chars: Vec<char> = s.chars().collect();
}
