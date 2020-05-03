use std::io::*;
use std::str::FromStr;

type NodeId = usize;

struct Node {
    parent: Option<NodeId>,
    left: Option<NodeId>,
    right: Option<NodeId>,
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

fn reconstruction(
    l: NodeId,
    r: NodeId,
    nodes: &mut Vec<Node>,
    preorder: &Vec<NodeId>,
    inorder: &Vec<NodeId>,
    position: &mut NodeId,
) {
    if l <= r {
        return;
    }
}

fn postorder(node_id: NodeId, nodes: &Vec<Node>) {
    if nodes[node_id].left != None {
        postorder(nodes[node_id].left.unwrap(), nodes)
    }
    if nodes[node_id].right != None {
        postorder(nodes[node_id].right.unwrap(), nodes)
    }
    print!(" {}", node_id);
}

fn main() {
    let n: usize = read();
    let mut nodes: Vec<Node> = (0..n)
        .map(|_| Node {
            parent: None,
            left: None,
            right: None,
        })
        .collect();

    let mut root_node_id: NodeId = 0;

    let preorder: Vec<NodeId> = (0..n)
        .map(|_| {
            let number: NodeId = read();
            return number - 1;
        })
        .collect();
    let inorder: Vec<NodeId> = (0..n)
        .map(|_| {
            let number: NodeId = read();
            return number - 1;
        })
        .collect();

    let mut position: NodeId = 0;

    reconstruction(0, n, &mut nodes, &preorder, &inorder, &mut position);

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

    println!("Preorder");
    println!();
    println!("Inorder");
    println!();
    println!("Postorder");
    postorder(root_node_id, &nodes);
    println!();
}
