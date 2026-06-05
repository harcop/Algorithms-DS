/// LeetCode #1740 - Find Distance in a Binary Tree
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn build_tree(vals: Vec<Option<i32>>) -> Option<Box<TreeNode>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let mut nodes: Vec<Option<Box<TreeNode>>> = vals
        .into_iter()
        .map(|v| v.map(|x| Box::new(TreeNode { val: x, left: None, right: None })))
        .collect();
    let n = nodes.len();
    for i in (0..n).rev() {
        if nodes[i].is_none() {
            continue;
        }
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        let left = if l < n { nodes[l].take() } else { None };
        let right = if r < n { nodes[r].take() } else { None };
        let node = nodes[i].as_mut().unwrap();
        node.left = left;
        node.right = right;
    }
    nodes[0].take()
}

fn find_distance(root: Option<Box<TreeNode>>, p: i32, q: i32) -> i32 {
    let root = root.as_ref().unwrap();
    let mut parent: HashMap<i32, i32> = HashMap::new();
    let mut depth: HashMap<i32, i32> = HashMap::new();
    let mut qd: VecDeque<(&TreeNode, i32)> = VecDeque::new();
    qd.push_back((root.as_ref(), 0));
    while let Some((node, d)) = qd.pop_front() {
        depth.insert(node.val, d);
        if let Some(l) = &node.left {
            parent.insert(l.val, node.val);
            qd.push_back((l.as_ref(), d + 1));
        }
        if let Some(r) = &node.right {
            parent.insert(r.val, node.val);
            qd.push_back((r.as_ref(), d + 1));
        }
    }
    let mut a = p;
    let mut b = q;
    while depth[&a] > depth[&b] {
        a = parent[&a];
    }
    while depth[&b] > depth[&a] {
        b = parent[&b];
    }
    while a != b {
        a = parent[&a];
        b = parent[&b];
    }
    depth[&p] + depth[&q] - 2 * depth[&a]
}

fn main() {
    let root = build_tree(vec![
        Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8),
        None, None, None, Some(7), Some(4),
    ]);
    println!("{}", find_distance(root, 5, 0));
}
#[cfg(test)]
mod tests {
    use super::{build_tree, find_distance, TreeNode};
    fn tree() -> Option<Box<TreeNode>> {
        build_tree(vec![
            Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8),
            None, None, None, Some(7), Some(4),
        ])
    }
    #[test]
    fn example_one() {
        assert_eq!(find_distance(tree(), 5, 0), 3);
    }
    #[test]
    fn example_two() {
        assert_eq!(find_distance(tree(), 5, 7), 2);
    }
}
