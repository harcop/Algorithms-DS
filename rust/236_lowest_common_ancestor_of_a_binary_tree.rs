/// LeetCode #236 - Lowest Common Ancestor of a Binary Tree (returns LCA value)
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn lowest_common_ancestor(root: &Option<Box<TreeNode>>, p: i32, q: i32) -> i32 {
    fn dfs(r: &Option<Box<TreeNode>>, p: i32, q: i32) -> Option<i32> {
        let n = r.as_ref()?;
        let l = dfs(&n.left, p, q);
        let rgt = dfs(&n.right, p, q);
        if l.is_some() && rgt.is_some() {
            return Some(n.val);
        }
        if n.val == p || n.val == q {
            return Some(n.val);
        }
        l.or(rgt)
    }
    dfs(root, p, q).unwrap()
}

fn main() {
    println!("{}", lowest_common_ancestor(&None, 0, 1));
}

#[cfg(test)]
mod tests {
    use super::{lowest_common_ancestor, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 2,
                    left: Some(Box::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 1,
                left: Some(Box::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 8,
                    left: None,
                    right: None,
                })),
            })),
        });
        assert_eq!(lowest_common_ancestor(&Some(root), 5, 1), 3);
    }

    #[test]
    fn example_two() {
        let root = Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 2,
                    left: Some(Box::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 1,
                left: Some(Box::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 8,
                    left: None,
                    right: None,
                })),
            })),
        });
        assert_eq!(lowest_common_ancestor(&Some(root), 5, 4), 5);
    }
}
