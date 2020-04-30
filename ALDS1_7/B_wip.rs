use std::cmp::max;
use std::io::*;
use std::str::FromStr;

type NodeId = usize;

struct Node {
    parent: Option<NodeId>,
    left: Option<NodeId>,
    right: Option<NodeId>,
    depth: u32,
    height: u32,
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

fn set_depth(node_id: NodeId, depth: u32, nodes: &mut Vec<Node>) {
    nodes[node_id].depth = depth;
    if nodes[node_id].left != None {
        set_depth(nodes[node_id].left.unwrap(), depth + 1, nodes)
    }
    if nodes[node_id].right != None {
        set_depth(nodes[node_id].right.unwrap(), depth + 1, nodes)
    }
}

fn set_height(node_id: NodeId, nodes: &mut Vec<Node>) -> u32 {
    let mut left_height: u32 = 0;
    let mut right_height: u32 = 0;
    if nodes[node_id].left != None {
        left_height = set_height(nodes[node_id].left.unwrap(), nodes) + 1;
    }
    if nodes[node_id].right != None {
        right_height = set_height(nodes[node_id].right.unwrap(), nodes) + 1;
    }
    nodes[node_id].height = max(left_height, right_height);
    return nodes[node_id].height;
}

fn main() {
    let n: usize = read();
    let mut nodes: Vec<Node> = (0..n)
        .map(|_| Node {
            parent: None,
            left: None,
            right: None,
            depth: 0,
            height: 0,
        })
        .collect();

    let mut root_node_id: NodeId = 0;

    for _ in 0..n {
        let node_id: NodeId = read();
        let left_input: i64 = read();
        let right_input: i64 = read();

        if left_input == -1 {
            nodes[node_id].left = None;
        } else {
            nodes[node_id].left = Some(left_input as usize);
            nodes[left_input as usize].parent = Some(node_id);
        }

        if right_input == -1 {
            nodes[node_id].right = None;
        } else {
            nodes[node_id].right = Some(right_input as usize);
            nodes[right_input as usize].parent = Some(node_id);
        }
    }

    for index in 0..n {
        if nodes[index].parent == None {
            root_node_id = index;
        }
    }

    set_depth(root_node_id, 0, &mut nodes);
    set_height(root_node_id, &mut nodes);

    for (index, node) in nodes.iter().enumerate() {
        print!("node {}: ", index);
        if node.parent == None {
            print!("parent = -1, ");
        } else {
            print!("parent = {}, ", node.parent.unwrap());
        }
        print!("depth = {}, ", node.depth);

        // TODO: sibling

        if node.parent == None {
            println!("root, ");
        } else if node.left == None && node.right == None {
            println!("leaf, ");
        } else {
            println!("internal node, ");
        }
    }
}
