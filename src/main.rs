use std::collections::HashMap;
use std::io::*;
use std::str::FromStr;

type NodeId = usize;

struct Node {
    id: NodeId,
    parent: Option<NodeId>,
    left: Option<NodeId>,
    right: Option<NodeId>,
    value: u32,
    target_char: Option<char>,
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

    fn make_nodes(&mut self, chars_count: HashMap<char, u32>) {
        for (current_char, value) in chars_count {
            let node = Node {
                id: self.nodes.len(),
                parent: None,
                value: value,
                target_char: Some(current_char),
                left: None,
                right: None,
            };
            self.nodes.push(node)
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
    let mut chars_count: HashMap<char, u32> = HashMap::new();
    let s_chars: Vec<char> = s.chars().collect();
    for c in s_chars {
        let count = chars_count.entry(c).or_insert(0);
        *count += 1;
        println!("{}, {}", c, count)
    }
    let mut chars_convert_map: HashMap<char, String> = HashMap::new();
    let mut tree = Tree::new();
    tree.make_nodes(chars_count);
    println!("{}", tree.nodes.len());
}
