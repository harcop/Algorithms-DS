/// LeetCode #617 - Merge Two Binary Trees
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn merge_trees(
    mut root1: Option<Box<TreeNode>>,
    mut root2: Option<Box<TreeNode>>,
) -> Option<Box<TreeNode>> {
    match (root1.take(), root2.take()) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(mut n1), Some(mut n2)) => {
            n1.val += n2.val;
            n1.left = merge_trees(n1.left.take(), n2.left.take());
            n1.right = merge_trees(n1.right.take(), n2.right.take());
            Some(n1)
        }
    }
}

fn main() {
    println!("{}", merge_trees(None, None).is_none());
}

#[cfg(test)]
mod tests {
    use super::{merge_trees, TreeNode};

    #[test]
    fn example_one() {
        let a = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 3,
                left: Some(Box::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
        }));
        let b = Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
            })),
        }));
        let r = merge_trees(a, b).unwrap();
        assert_eq!(r.val, 3);
    }
}
