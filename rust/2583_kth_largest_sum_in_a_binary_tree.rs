/// LeetCode #2583 - Kth Largest Sum in a Binary Tree
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn tree_from_vals(vals: Vec<Option<i32>>) -> Option<Box<TreeNode>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let mut nodes: Vec<Option<Box<TreeNode>>> = vals
        .into_iter()
        .map(|v| {
            v.map(|x| {
                Box::new(TreeNode {
                    val: x,
                    left: None,
                    right: None,
                })
            })
        })
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

fn kth_largest_level_sum(root: Option<Box<TreeNode>>, k: i32) -> i64 {
    let Some(root) = root else {
        return -1;
    };
    let mut arr = Vec::new();
    let mut q = VecDeque::new();
    q.push_back(root);
    while !q.is_empty() {
        let mut t = 0i64;
        for _ in 0..q.len() {
            let node = q.pop_front().unwrap();
            t += node.val as i64;
            if let Some(left) = node.left {
                q.push_back(left);
            }
            if let Some(right) = node.right {
                q.push_back(right);
            }
        }
        arr.push(t);
    }
    let k = k as usize;
    if arr.len() < k {
        return -1;
    }
    arr.sort_unstable_by(|a, b| b.cmp(a));
    arr[k - 1]
}

fn main() {
    let root = tree_from_vals(vec![
        Some(5),
        Some(8),
        Some(9),
        Some(2),
        Some(1),
        Some(3),
        Some(7),
        Some(4),
        Some(6),
    ]);
    println!("{}", kth_largest_level_sum(root, 2));
}

#[cfg(test)]
mod tests {
    use super::{kth_largest_level_sum, tree_from_vals};

    #[test]
    fn example_one() {
        let root = tree_from_vals(vec![
            Some(5),
            Some(8),
            Some(9),
            Some(2),
            Some(1),
            Some(3),
            Some(7),
            Some(4),
            Some(6),
        ]);
        assert_eq!(kth_largest_level_sum(root, 2), 13);
    }

    #[test]
    fn example_two() {
        let root = tree_from_vals(vec![Some(1), Some(2), None, Some(3)]);
        assert_eq!(kth_largest_level_sum(root, 1), 3);
    }
}
