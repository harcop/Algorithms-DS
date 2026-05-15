/// LeetCode #783 - Minimum Distance Between BST Nodes
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn min_diff_in_bst(root: Option<Box<TreeNode>>) -> i32 {
    let mut prev: Option<i32> = None;
    let mut ans = i32::MAX;
    fn dfs(n: &Option<Box<TreeNode>>, prev: &mut Option<i32>, ans: &mut i32) {
        if let Some(node) = n {
            dfs(&node.left, prev, ans);
            if let Some(p) = *prev {
                *ans = (*ans).min(node.val - p);
            }
            *prev = Some(node.val);
            dfs(&node.right, prev, ans);
        }
    }
    dfs(&root, &mut prev, &mut ans);
    ans
}

fn main() {
    let r = Box::new(TreeNode {
        val: 4,
        left: Some(Box::new(TreeNode {
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
        })),
        right: Some(Box::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        })),
    });
    println!("{}", min_diff_in_bst(Some(r)));
}

#[cfg(test)]
mod tests {
    use super::{min_diff_in_bst, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 4,
            left: Some(Box::new(TreeNode {
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
            })),
            right: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            })),
        }));
        assert_eq!(min_diff_in_bst(root), 1);
    }
}
