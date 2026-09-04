/// LeetCode #742 - Closest Leaf in a Binary Tree
use std::collections::{HashMap, HashSet, VecDeque};

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

fn closest_leaf(root: Option<Box<TreeNode>>, k: i32) -> i32 {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut leaves = HashSet::new();
    build(root.as_ref(), &mut graph, &mut leaves);
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(k);
    seen.insert(k);
    while let Some(u) = q.pop_front() {
        if leaves.contains(&u) {
            return u;
        }
        if let Some(nbrs) = graph.get(&u) {
            for &v in nbrs {
                if seen.insert(v) {
                    q.push_back(v);
                }
            }
        }
    }
    k
}

fn build(
    node: Option<&Box<TreeNode>>,
    graph: &mut HashMap<i32, Vec<i32>>,
    leaves: &mut HashSet<i32>,
) {
    let Some(n) = node else {
        return;
    };
    graph.entry(n.val).or_default();
    let mut is_leaf = true;
    if let Some(left) = n.left.as_ref() {
        is_leaf = false;
        graph.entry(n.val).or_default().push(left.val);
        graph.entry(left.val).or_default().push(n.val);
        build(n.left.as_ref(), graph, leaves);
    }
    if let Some(right) = n.right.as_ref() {
        is_leaf = false;
        graph.entry(n.val).or_default().push(right.val);
        graph.entry(right.val).or_default().push(n.val);
        build(n.right.as_ref(), graph, leaves);
    }
    if is_leaf {
        leaves.insert(n.val);
    }
}

fn main() {
    let mut root = Box::new(TreeNode::new(1));
    root.left = Some(Box::new(TreeNode::new(3)));
    root.right = Some(Box::new(TreeNode::new(2)));
    println!("{}", closest_leaf(Some(root), 1));
}

#[cfg(test)]
mod tests {
    use super::{closest_leaf, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(3)));
        root.right = Some(Box::new(TreeNode::new(2)));
        let ans = closest_leaf(Some(root), 1);
        assert!(ans == 2 || ans == 3);
    }

    #[test]
    fn example_two() {
        let root = Box::new(TreeNode::new(1));
        assert_eq!(closest_leaf(Some(root), 1), 1);
    }

    #[test]
    fn example_three() {
        let mut n5 = Box::new(TreeNode::new(5));
        n5.right = Some(Box::new(TreeNode::new(6)));
        let mut n4 = Box::new(TreeNode::new(4));
        n4.left = Some(n5);
        let mut n2 = Box::new(TreeNode::new(2));
        n2.left = Some(n4);
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(n2);
        root.right = Some(Box::new(TreeNode::new(3)));
        assert_eq!(closest_leaf(Some(root), 2), 3);
    }
}
