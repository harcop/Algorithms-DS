/// LeetCode #2236 - Root Equals Sum of Children
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn check_tree(root: Option<Box<TreeNode>>) -> bool {
    let Some(node) = root else {
        return false;
    };
    node.val == node.left.as_ref().unwrap().val + node.right.as_ref().unwrap().val
}

fn main() {
    let root = Box::new(TreeNode {
        val: 10,
        left: Some(Box::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        })),
    });
    println!("{}", check_tree(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{check_tree, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 10,
            left: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            })),
        });
        assert!(check_tree(Some(root)));
    }

    #[test]
    fn example_two() {
        let root = Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
        });
        assert!(!check_tree(Some(root)));
    }
}
