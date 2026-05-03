/// LeetCode #226 - Invert Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn invert_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let mut root = root?;
    let l = root.left.take();
    let r = root.right.take();
    root.left = invert_tree(r);
    root.right = invert_tree(l);
    Some(root)
}

fn main() {
    println!("{:?}", invert_tree(None));
}

#[cfg(test)]
mod tests {
    use super::{invert_tree, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
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
                val: 7,
                left: Some(Box::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                })),
            })),
        });
        let out = invert_tree(Some(root)).unwrap();
        assert_eq!(out.val, 4);
        assert_eq!(out.left.as_ref().unwrap().val, 7);
        assert_eq!(out.right.as_ref().unwrap().val, 2);
    }
}
