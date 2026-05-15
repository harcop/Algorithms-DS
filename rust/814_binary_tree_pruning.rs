/// LeetCode #814 - Binary Tree Pruning
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn prune_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    fn dfs(n: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match n {
            None => None,
            Some(mut node) => {
                node.left = dfs(node.left);
                node.right = dfs(node.right);
                if node.val == 1 || node.left.is_some() || node.right.is_some() {
                    Some(node)
                } else {
                    None
                }
            }
        }
    }
    dfs(root)
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 0,
            left: Some(Box::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            })),
        })),
    }));
    let _ = prune_tree(root);
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::{prune_tree, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 0,
                left: Some(Box::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 1,
                left: Some(Box::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                })),
            })),
        }));
        let out = prune_tree(root);
        assert!(out.is_some());
        let r = out.unwrap();
        assert_eq!(r.val, 1);
        assert!(r.left.is_none());
    }
}
