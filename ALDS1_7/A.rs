use std::io::*;
use std::str::FromStr;

type NodeId = usize;

struct Node {
    parent: Option<NodeId>,
    left_child: Option<NodeId>,
    right_brother: Option<NodeId>,
    depth: u32,
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

fn rec(node_id: NodeId, depth: u32, nodes: &mut Vec<Node>) {
    nodes[node_id].depth = depth;
    if nodes[node_id].right_brother != None {
        rec(nodes[node_id].right_brother.unwrap(), depth, nodes)
    }
    if nodes[node_id].left_child != None {
        rec(nodes[node_id].left_child.unwrap(), depth + 1, nodes)
    }
}

fn main() {
    let n: usize = read();
    let mut nodes: Vec<Node> = (0..n)
        .map(|_| Node {
            parent: None,
            left_child: None,
            right_brother: None,
            depth: 0,
        })
        .collect();

    let mut root_node_id: NodeId = 0;

    for _ in 0..n {
        let node_id: NodeId = read();
        let k: usize = read();
        let mut left_brother_id: Option<NodeId> = None;
        for index in 0..k {
            let child_node_id: NodeId = read();
            if index == 0 {
                nodes[node_id].left_child = Some(child_node_id);
            } else {
                nodes[left_brother_id.unwrap()].right_brother = Some(child_node_id);
            }
            left_brother_id = Some(child_node_id);
            nodes[child_node_id].parent = Some(node_id);
        }
    }

    for index in 0..n {
        if nodes[index].parent == None {
            root_node_id = index;
        }
    }

    rec(root_node_id, 0, &mut nodes);

    for (index, node) in nodes.iter().enumerate() {
        print!("node {}: ", index);
        if node.parent == None {
            print!("parent = -1, ");
        } else {
            print!("parent = {}, ", node.parent.unwrap());
        }
        print!("depth = {}, ", node.depth);

        if node.parent == None {
            print!("root, ");
        } else if node.left_child == None {
            print!("leaf, ");
        } else {
            print!("internal node, ");
        }

        print!("[");
        if node.left_child != None {
            let mut count = 0;
            let mut right_brother_id: Option<NodeId> = node.left_child;
            while right_brother_id != None {
                let right_brother: &Node = &nodes[right_brother_id.unwrap()];
                if count != 0 {
                    print!(", ")
                }
                print!("{}", right_brother_id.unwrap());
                count += 1;
                right_brother_id = right_brother.right_brother
            }
        }
        println!("]");
    }
}
