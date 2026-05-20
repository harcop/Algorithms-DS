/// LeetCode #1161 - Maximum Level Sum of a Binary Tree
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn max_level_sum(root: Option<Box<TreeNode>>) -> i32 {
    let mut q = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    let mut level = 1i32;
    let mut best_level = 1i32;
    let mut best_sum = i32::MIN;
    while !q.is_empty() {
        let sz = q.len();
        let mut sum = 0i32;
        for _ in 0..sz {
            let node = q.pop_front().unwrap();
            sum += node.val;
            if let Some(l) = node.left {
                q.push_back(l);
            }
            if let Some(r) = node.right {
                q.push_back(r);
            }
        }
        if sum > best_sum {
            best_sum = sum;
            best_level = level;
        }
        level += 1;
    }
    best_level
}

fn main() {
    println!("{}", max_level_sum(None));
}

#[cfg(test)]
mod tests {
    use super::{max_level_sum, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 7,
                left: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: -8,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        });
        assert_eq!(max_level_sum(Some(root)), 2);
    }

    #[test]
    fn example_two() {
        let root = Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
        });
        assert_eq!(max_level_sum(Some(root)), 2);
    }
}
