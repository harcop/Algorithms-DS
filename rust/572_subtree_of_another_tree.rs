/// LeetCode #572 - Subtree of Another Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn is_subtree(root: Option<Box<TreeNode>>, sub_root: Option<Box<TreeNode>>) -> bool {
    fn same(a: &Option<Box<TreeNode>>, b: &Option<Box<TreeNode>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(x), Some(y)) => {
                x.val == y.val && same(&x.left, &y.left) && same(&x.right, &y.right)
            }
            _ => false,
        }
    }
    fn dfs(r: &Option<Box<TreeNode>>, t: &Option<Box<TreeNode>>) -> bool {
        if same(r, t) {
            return true;
        }
        if let Some(n) = r {
            return dfs(&n.left, t) || dfs(&n.right, t);
        }
        false
    }
    dfs(&root, &sub_root)
}

fn main() {
    println!("{}", is_subtree(None, None));
}

#[cfg(test)]
mod tests {
    use super::{is_subtree, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 4,
                left: Some(Box::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            })),
        }));
        let sub = Some(Box::new(TreeNode {
            val: 4,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
        }));
        assert!(is_subtree(root, sub));
    }
}
