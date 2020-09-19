use std::collections::HashMap;
use std::io::*;
use std::str::FromStr;

type NodeId = usize;

struct Node {
    id: NodeId,
    parent: Option<NodeId>,
    left: Option<NodeId>,
    right: Option<NodeId>,
    value: String,
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

    fn make_nodes(&mut self, value: String, parent: Option<NodeId>) {
        let node = Node {
            id: self.nodes.len(),
            parent: parent,
            value: value,
            left: None,
            right: None,
        };

        self.nodes.push(node)
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
    let mut chars_count: HashMap<char, u32> = HashMap::new();
    let mut chars_convert_map: HashMap<char, String> = HashMap::new();
    let mut tree = Tree::new();
    for c in s_chars {
        let count = chars_count.entry(c).or_insert(0);
        *count += 1;
        println!("{}, {}", c, count)
    }
    tree.make_nodes(s, None);
}
