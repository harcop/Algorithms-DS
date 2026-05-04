/// LeetCode #285 - Inorder Successor in BST
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn inorder_successor(root: Option<Box<TreeNode>>, p: i32) -> i32 {
    let mut succ = None;
    let mut cur = root.as_ref();
    while let Some(n) = cur {
        if n.val > p {
            succ = Some(n.val);
            cur = n.left.as_ref();
        } else {
            cur = n.right.as_ref();
        }
    }
    succ.unwrap_or(-1)
}

fn main() {
    let root = Box::new(TreeNode {
        val: 2,
        left: Some(Box::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })),
        right: None,
    });
    println!("{}", inorder_successor(Some(root), 1));
}

#[cfg(test)]
mod tests {
    use super::{inorder_successor, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
        });
        assert_eq!(inorder_successor(Some(root), 1), 2);
    }
}
