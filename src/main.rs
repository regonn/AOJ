use std::collections::HashMap;
use std::io::*;
use std::str::FromStr;

type NodeId = usize;

#[derive(Copy, Clone, Debug, PartialEq)]
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

    fn no_parent_nodes(&mut self) -> Vec<Node> {
        let mut no_parent_nodes: Vec<Node> = vec![];
        for node in self.nodes.clone().iter() {
            if node.parent.is_none() {
                no_parent_nodes.push(*node);
            }
        }
        no_parent_nodes.sort_by(|a, b| a.value.cmp(&b.value));
        return no_parent_nodes;
    }

    fn no_parent_nodes_len(&mut self) -> usize {
        return self.no_parent_nodes().len();
    }

    fn make_node(&mut self) -> Node {
        let no_parent_nodes = self.no_parent_nodes();
        let left = no_parent_nodes[0];
        let right = no_parent_nodes[1];

        let node = Node {
            id: self.nodes.len(),
            parent: None,
            value: left.value + right.value,
            target_char: None,
            left: Some(left.id),
            right: Some(right.id),
        };

        return node;
    }

    fn make(&mut self) {
        while self.no_parent_nodes_len() != 1 {
            let node: Node = self.make_node();
            self.nodes.push(node);
            self.nodes[node.left.unwrap()].parent = Some(node.id);
            self.nodes[node.right.unwrap()].parent = Some(node.id);
        }
        self.root = Some(self.no_parent_nodes()[0].id);
    }

    fn insert_map(
        &mut self,
        chars_convert_map: &mut HashMap<char, String>,
        node_id: usize,
        value: String,
    ) {
        let node = self.nodes[node_id];
        if !node.left.is_none() {
            let left = self.nodes[node.left.unwrap()];
            let node_value = value.clone() + "0";
            if left.target_char.is_none() {
                self.insert_map(chars_convert_map, left.id, node_value);
            } else {
                chars_convert_map
                    .entry(left.target_char.unwrap())
                    .or_insert(node_value.clone());
            }
        }
        if !node.right.is_none() {
            let right = self.nodes[node.right.unwrap()];
            let node_value = value.clone() + "1";
            if right.target_char.is_none() {
                self.insert_map(chars_convert_map, right.id, node_value);
            } else {
                chars_convert_map
                    .entry(right.target_char.unwrap())
                    .or_insert(node_value.clone());
            }
        }
    }

    fn make_convert_map(&mut self) -> HashMap<char, String> {
        let mut chars_convert_map: HashMap<char, String> = HashMap::new();
        let root = self.root.unwrap();
        if self.nodes.len() == 1 {
            let node = self.nodes[0];
            chars_convert_map
                .entry(node.target_char.unwrap())
                .or_insert("0".to_string());
        } else {
            self.insert_map(&mut chars_convert_map, root, "".to_string());
        }
        return chars_convert_map;
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
    for c in s_chars.clone() {
        let count = chars_count.entry(c).or_insert(0);
        *count += 1;
    }
    let mut tree = Tree::new();
    tree.make_nodes(chars_count);
    tree.make();
    let chars_convert_map: HashMap<char, String> = tree.make_convert_map();
    let mut answer: usize = 0;
    for c in s_chars.clone() {
        let binary_chars: Vec<char> = chars_convert_map.get(&c).unwrap().chars().collect();
        answer += binary_chars.len();
    }
    println!("{}", answer);
}
