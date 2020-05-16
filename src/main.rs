use std::io::*;
use std::str::FromStr;

type NodeId = usize;

struct Node {
    id: NodeId,
    parent: Option<NodeId>,
    left: Option<NodeId>,
    right: Option<NodeId>,
    key: u64,
    priority: u64,
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

    fn right_rotate(&mut self, node_id: NodeId) -> NodeId {
        let left_node_id = self.nodes[node_id].left.unwrap();
        self.nodes[node_id].left = self.nodes[left_node_id].right;
        self.nodes[left_node_id].right = Some(node_id);
        return left_node_id;
    }

    fn left_rotate(&mut self, node_id: NodeId) -> NodeId {
        let right_node_id = self.nodes[node_id].right.unwrap();
        self.nodes[node_id].right = self.nodes[right_node_id].left;
        self.nodes[right_node_id].left = Some(node_id);
        return right_node_id;
    }

    fn insert(&mut self, node_id: Option<NodeId>, key: u64, priority: u64) -> Option<NodeId> {
        if node_id == None {
            let id: NodeId = self.nodes.len();
            let node = Node {
                id: id,
                parent: None,
                left: None,
                right: None,
                key: key,
                priority: priority,
            };
            self.nodes.push(node);
            return Some(id);
        }

        let mut current_node_id = node_id.unwrap();

        if key == self.nodes[current_node_id].key {
            return Some(current_node_id);
        }

        if key < self.nodes[current_node_id].key {
            let mut left_node_id: Option<NodeId> = self.nodes[current_node_id].left;
            self.nodes[current_node_id].left = self.insert(left_node_id, key, priority);
            left_node_id = self.nodes[current_node_id].left;
            if self.nodes[current_node_id].priority < self.nodes[left_node_id.unwrap()].priority {
                current_node_id = self.right_rotate(current_node_id);
            }
        } else {
            let mut right_node_id: Option<NodeId> = self.nodes[current_node_id].right;
            self.nodes[current_node_id].right = self.insert(right_node_id, key, priority);
            right_node_id = self.nodes[current_node_id].right;
            if self.nodes[current_node_id].priority < self.nodes[right_node_id.unwrap()].priority {
                current_node_id = self.left_rotate(current_node_id);
            }
        }

        return Some(current_node_id);
    }

    fn find(&self, key: u64) -> Option<NodeId> {
        if self.root == None {
            return None;
        } else {
            let mut node_id: Option<NodeId> = Some(self.nodes[self.root.unwrap()].id);
            while node_id != None {
                if self.nodes[node_id.unwrap()].key == key {
                    return node_id;
                } else if self.nodes[node_id.unwrap()].key < key {
                    node_id = self.nodes[node_id.unwrap()].right;
                } else {
                    node_id = self.nodes[node_id.unwrap()].left;
                }
            }
            return None;
        }
    }

    fn get_minimum(&mut self, node_id: NodeId) -> NodeId {
        let mut return_node_id: NodeId = node_id;
        while self.nodes[return_node_id].left != None {
            return_node_id = self.nodes[return_node_id].left.unwrap();
        }
        return return_node_id;
    }

    fn get_next_inorder(&mut self, node_id: NodeId) -> NodeId {
        let mut current_node_id: NodeId = node_id;
        if self.nodes[current_node_id].right != None {
            let right_child_id = self.nodes[current_node_id].right.unwrap();
            return self.get_minimum(right_child_id);
        }

        let mut parent_id: Option<NodeId> = self.nodes[current_node_id].parent;
        while parent_id != None
            && Some(self.nodes[current_node_id].id) == self.nodes[parent_id.unwrap()].right
        {
            current_node_id = parent_id.unwrap();
            parent_id = self.nodes[current_node_id].parent;
        }
        return current_node_id;
    }

    fn delete(&mut self, key: u64) {
        let node_id: Option<NodeId> = self.find(key);
        if node_id != None {
            let current_node_id: NodeId = node_id.unwrap();
            let delete_target_node_id: NodeId;
            let delete_target_child_id: Option<NodeId>;
            if self.nodes[current_node_id].left == None || self.nodes[current_node_id].right == None
            {
                delete_target_node_id = current_node_id;
            } else {
                delete_target_node_id = self.get_next_inorder(current_node_id);
            }

            if self.nodes[delete_target_node_id].left != None {
                delete_target_child_id = self.nodes[delete_target_node_id].left;
            } else {
                delete_target_child_id = self.nodes[delete_target_node_id].right;
            }

            if delete_target_child_id != None {
                self.nodes[delete_target_child_id.unwrap()].parent =
                    self.nodes[delete_target_node_id].parent
            }

            if self.nodes[delete_target_node_id].parent == None {
                self.root = delete_target_child_id;
            } else {
                let delete_target_parent_id: NodeId =
                    self.nodes[delete_target_node_id].parent.unwrap();
                if Some(delete_target_node_id) == self.nodes[delete_target_parent_id].left {
                    self.nodes[delete_target_parent_id].left = delete_target_child_id;
                } else {
                    self.nodes[delete_target_parent_id].right = delete_target_child_id;
                }
            }

            if current_node_id != delete_target_node_id {
                self.nodes[current_node_id].key = self.nodes[delete_target_node_id].key;
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
    print!(" {}", nodes[node_id].key);
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
    print!(" {}", nodes[node_id].key);
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
                let key: u64 = read();
                let priority: u64 = read();
                tree.insert(None, key, priority);
            }
            "find" => {
                let key: u64 = read();
                if tree.find(key) != None {
                    println!("yes");
                } else {
                    println!("no");
                };
            }
            "delete" => {
                let key: u64 = read();
                tree.delete(key);
            }
            _ => tree.print(),
        }
    }
}
