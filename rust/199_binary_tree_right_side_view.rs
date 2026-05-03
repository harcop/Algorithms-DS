/// LeetCode #199 - Binary Tree Right Side View
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn right_side_view(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    let mut q = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    while !q.is_empty() {
        let k = q.len();
        for i in 0..k {
            let mut n = q.pop_front().unwrap();
            if i + 1 == k {
                out.push(n.val);
            }
            if let Some(l) = n.left.take() {
                q.push_back(l);
            }
            if let Some(r) = n.right.take() {
                q.push_back(r);
            }
        }
    }
    out
}

fn main() {
    println!("{:?}", right_side_view(None));
}

#[cfg(test)]
mod tests {
    use super::{right_side_view, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                })),
            })),
        });
        assert_eq!(right_side_view(Some(root)), vec![1, 3, 5]);
    }
}
