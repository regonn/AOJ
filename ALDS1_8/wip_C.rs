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

    fn insert(&mut self, value: i64) {
        let mut y: Option<NodeId> = None;
        let mut x: Option<NodeId> = self.root;

        while x != None {
            y = x;
            if value < self.nodes[x.unwrap()].value {
                x = self.nodes[x.unwrap()].left;
            } else {
                x = self.nodes[x.unwrap()].right;
            }
        }

        let node = Node {
            id: self.nodes.len(),
            parent: y,
            value: value,
            left: None,
            right: None,
        };

        if y == None {
            self.root = Some(0);
        } else if node.value < self.nodes[y.unwrap()].value {
            self.nodes[y.unwrap()].left = Some(node.id);
        } else {
            self.nodes[y.unwrap()].right = Some(node.id);
        }

        self.nodes.push(node)
    }

    fn find(&self, value: i64) -> Option<NodeId> {
        if self.root == None {
            return None;
        } else {
            let mut node_id: Option<NodeId> = Some(self.nodes[self.root.unwrap()].id);
            while node_id != None {
                if self.nodes[node_id.unwrap()].value == value {
                    return node_id;
                } else if self.nodes[node_id.unwrap()].value < value {
                    node_id = self.nodes[node_id.unwrap()].right;
                } else {
                    node_id = self.nodes[node_id.unwrap()].left;
                }
            }
            return None;
        }
    }

    fn delete(&mut self, value: i64) {
        let node_id: Option<NodeId> = self.find(value);
        if node_id != None {
            let current_node_id: NodeId = node_id.unwrap();
            if self.nodes[current_node_id].left == None && self.nodes[current_node_id].right == None
            {
                let parent_node_id: NodeId = self.nodes[current_node_id].parent.unwrap();
                if self.nodes[parent_node_id].left == node_id {
                    self.nodes[parent_node_id].left = None;
                } else {
                    self.nodes[parent_node_id].right = None;
                }
            } else if self.nodes[current_node_id].left != None
                && self.nodes[current_node_id].right != None
            {
                let left_child_id: NodeId = self.nodes[current_node_id].left.unwrap();
                let left_child_value: i64 = self.nodes[left_child_id].value;
                self.delete(left_child_value);
                self.nodes[current_node_id].value = left_child_value;
            } else {
                if self.nodes[current_node_id].left != None {
                    let child_id: NodeId = self.nodes[current_node_id].left.unwrap();
                    let parent_node_id: NodeId = self.nodes[current_node_id].parent.unwrap();
                    // TODO: node has one child
                }
            }
        }
    }

    fn print(&mut self) {
        inorder(0, &self.nodes);
        println!();
        preorder(0, &self.nodes);
        println!();
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
        match &*command {
            "insert" => {
                let number: i64 = read();
                tree.insert(number);
            }
            "find" => {
                let number: i64 = read();
                if tree.find(number) != None {
                    println!("yes");
                } else {
                    println!("no");
                };
            }
            "delete" => {
                let number: i64 = read();
                tree.delete(number);
            }
            _ => tree.print(),
        }
    }
}
