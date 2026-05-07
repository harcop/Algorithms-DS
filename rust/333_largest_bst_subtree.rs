/// LeetCode #333 - Largest BST Subtree
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn leaf(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn largest_bst_subtree(root: Option<Box<TreeNode>>) -> i32 {
    let mut ans = 0i32;

    fn go(n: &Option<Box<TreeNode>>, ans: &mut i32) -> (i64, i64, i32) {
        match n {
            None => (i64::MAX, i64::MIN, 0),
            Some(node) => {
                let (lmin, lmax, ls) = go(&node.left, ans);
                let (rmin, rmax, rs) = go(&node.right, ans);
                if ls >= 0
                    && rs >= 0
                    && node.val as i64 > lmax
                    && (node.val as i64) < rmin
                {
                    let sz = ls + rs + 1;
                    *ans = (*ans).max(sz);
                    (lmin.min(node.val as i64), rmax.max(node.val as i64), sz)
                } else {
                    (-1, -1, -1)
                }
            }
        }
    }

    go(&root, &mut ans);
    ans
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 10,
        left: Some(Box::new(TreeNode::leaf(5))),
        right: Some(Box::new(TreeNode {
            val: 15,
            left: Some(Box::new(TreeNode::leaf(14))),
            right: Some(Box::new(TreeNode::leaf(18))),
        })),
    }));
    println!("{}", largest_bst_subtree(root));
}

#[cfg(test)]
mod tests {
    use super::{largest_bst_subtree, TreeNode};

    #[test]
    fn leet_example1() {
        let root = Some(Box::new(TreeNode {
            val: 10,
            left: Some(Box::new(TreeNode::leaf(5))),
            right: Some(Box::new(TreeNode {
                val: 15,
                left: Some(Box::new(TreeNode::leaf(14))),
                right: Some(Box::new(TreeNode::leaf(18))),
            })),
        }));
        // Inorder 5,10,14,15,18 is sorted; entire tree is a BST of size 5.
        assert_eq!(largest_bst_subtree(root), 5);
    }

    #[test]
    fn all_bst() {
        let root = Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode::leaf(1))),
            right: Some(Box::new(TreeNode::leaf(4))),
        }));
        assert_eq!(largest_bst_subtree(root), 3);
    }
}
