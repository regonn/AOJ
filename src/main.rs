use std::io::*;
use std::str::FromStr;

type NodeId = usize;

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
    preorder: &Vec<NodeId>,
    inorder: &Vec<NodeId>,
    postorder: &mut Vec<NodeId>,
    position: &mut NodeId,
) {
    if l >= r {
        return;
    }
    *position += 1;
    let root: NodeId = preorder[*position];
    let m: NodeId = inorder.iter().position(|&x| x == root).unwrap();
    reconstruction(l, m, preorder, inorder, postorder, position);
    reconstruction(m + 1, r, preorder, inorder, postorder, position);
    postorder.push(root);
}

fn main() {
    let n: usize = read();

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

    let mut postorder: Vec<NodeId> = vec![];

    let mut position: NodeId = 0;

    reconstruction(0, n, &preorder, &inorder, &mut postorder, &mut position);

    for (index, number) in postorder.iter().enumerate() {
        if index != n - 1 {
            print!("{} ", number + 1);
        } else {
            println!("{}", number + 1);
        }
    }
}
