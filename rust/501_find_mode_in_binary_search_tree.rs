/// LeetCode #501 - Find Mode in Binary Search Tree
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn find_mode(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut counts = HashMap::new();
    dfs(&root, &mut counts);
    let max = counts.values().copied().max().unwrap_or(0);
    counts
        .into_iter()
        .filter(|(_, c)| *c == max)
        .map(|(v, _)| v)
        .collect()
}

fn dfs(node: &Option<Box<TreeNode>>, counts: &mut HashMap<i32, i32>) {
    if let Some(n) = node {
        *counts.entry(n.val).or_insert(0) += 1;
        dfs(&n.left, counts);
        dfs(&n.right, counts);
    }
}

fn main() {
    let mut root = Box::new(TreeNode::new(1));
    let mut r = Box::new(TreeNode::new(2));
    r.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(r);
    println!("{:?}", find_mode(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{find_mode, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(1));
        let mut r = Box::new(TreeNode::new(2));
        r.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(r);
        let mut got = find_mode(Some(root));
        got.sort_unstable();
        assert_eq!(got, vec![2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_mode(Some(Box::new(TreeNode::new(0)))), vec![0]);
    }
}
