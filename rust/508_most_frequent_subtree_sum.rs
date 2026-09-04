/// LeetCode #508 - Most Frequent Subtree Sum
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

fn find_frequent_tree_sum(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut freq = HashMap::new();
    dfs(&root, &mut freq);
    let max = freq.values().copied().max().unwrap_or(0);
    freq.into_iter()
        .filter(|(_, c)| *c == max)
        .map(|(s, _)| s)
        .collect()
}

fn dfs(node: &Option<Box<TreeNode>>, freq: &mut HashMap<i32, i32>) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let s = n.val + dfs(&n.left, freq) + dfs(&n.right, freq);
            *freq.entry(s).or_insert(0) += 1;
            s
        }
    }
}

fn main() {
    let mut root = Box::new(TreeNode::new(5));
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(-3)));
    println!("{:?}", find_frequent_tree_sum(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{find_frequent_tree_sum, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(5));
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(-3)));
        let mut got = find_frequent_tree_sum(Some(root));
        got.sort_unstable();
        assert_eq!(got, vec![-3, 2, 4]);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(5));
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(-5)));
        assert_eq!(find_frequent_tree_sum(Some(root)), vec![2]);
    }
}
