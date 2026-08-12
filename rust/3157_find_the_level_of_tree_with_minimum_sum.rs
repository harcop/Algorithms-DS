/// LeetCode #3157 - Find the Level of Tree with Minimum Sum
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn minimum_level(root: Option<Box<TreeNode>>) -> i32 {
    let Some(root) = root else {
        return 0;
    };
    let mut q = VecDeque::new();
    q.push_back(root);
    let mut ans = 1;
    let mut best = i64::MAX;
    let mut level = 1;
    while !q.is_empty() {
        let mut sum = 0i64;
        for _ in 0..q.len() {
            let node = q.pop_front().unwrap();
            sum += node.val as i64;
            if let Some(l) = node.left {
                q.push_back(l);
            }
            if let Some(r) = node.right {
                q.push_back(r);
            }
        }
        if sum < best {
            best = sum;
            ans = level;
        }
        level += 1;
    }
    ans
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 50,
        left: Some(Box::new(TreeNode {
            val: 6,
            left: Some(Box::new(TreeNode {
                val: 30,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 80,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
            right: None,
        })),
    }));
    println!("{}", minimum_level(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root = Some(Box::new(TreeNode {
            val: 50,
            left: Some(Box::new(TreeNode {
                val: 6,
                left: Some(Box::new(TreeNode {
                    val: 30,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 80,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        }));
        assert_eq!(minimum_level(root), 2);
    }

    #[test]
    fn example3() {
        let root = Some(Box::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                })),
            })),
        }));
        assert_eq!(minimum_level(root), 1);
    }
}
