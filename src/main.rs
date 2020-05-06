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

    fn insert(&mut self, value: i64) {
        let mut y: Option<NodeId> = None;
        let mut x: Option<NodeId> = self.root;

        while x != None {}

        let mut node = Node {
            parent: y,
            value: value,
            left: None,
            right: None,
        };
    }

    fn print(&mut self) {
        inorder(0, &self.nodes);
        preorder(0, &self.nodes);
    }
}

fn preorder(node_id: NodeId, nodes: &Vec<Node>) {
    print!(" {}", nodes[node_id].value);
    if nodes[node_id].left != None {
        preorder(nodes[node_id].left.unwrap(), nodes)
    }
    if nodes[node_id].right != None {
        preorder(nodes[node_id].right.unwrap(), nodes)
    }
}

fn inorder(node_id: NodeId, nodes: &Vec<Node>) {
    if nodes[node_id].left != None {
        inorder(nodes[node_id].left.unwrap(), nodes)
    }
    print!(" {}", nodes[node_id].value);
    if nodes[node_id].right != None {
        inorder(nodes[node_id].right.unwrap(), nodes)
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
    let n: usize = read();
    let mut tree: Tree = Tree::new();
    for _ in 0..n {
        let command: String = read();
        if command == "insert" {
            let number: i64 = read();
            tree.insert(number);
        } else if command == "print" {
            tree.print();
        }
    }
}
