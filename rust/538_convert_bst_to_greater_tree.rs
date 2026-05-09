/// LeetCode #538 - Convert BST to Greater Tree
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn convert_bst(mut root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let mut sum = 0i32;
    fn dfs(node: &mut Option<Box<TreeNode>>, sum: &mut i32) {
        let Some(n) = node else { return };
        dfs(&mut n.right, sum);
        *sum += n.val;
        n.val = *sum;
        dfs(&mut n.left, sum);
    }
    dfs(&mut root, &mut sum);
    root
}

fn main() {
    println!("{}", convert_bst(None).is_none());
}

#[cfg(test)]
mod tests {
    use super::{convert_bst, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Some(Box::new(TreeNode {
            val: 4,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: Some(Box::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Box::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 6,
                left: Some(Box::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: Some(Box::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None,
                    })),
                })),
            })),
        }));
        root = convert_bst(root);
        let r = root.unwrap();
        assert_eq!(r.val, 30);
    }
}
