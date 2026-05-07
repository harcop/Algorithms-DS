/// LeetCode #426 - Convert BST to Sorted Doubly Linked List  
/// This version materializes a sorted **singly** right-threaded chain with `left=None` (BST values in order).  
/// Doubly/circular wiring with `Box<TreeNode>` alone would require intrusive raw pointers; this keeps the file safe and test-friendly.
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn tree_to_doubly_list(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let mut vals = vec![];
    fn inorder(o: Option<Box<TreeNode>>, acc: &mut Vec<i32>) {
        if let Some(mut n) = o {
            inorder(n.left.take(), acc);
            acc.push(n.val);
            inorder(n.right.take(), acc);
        }
    }
    inorder(root, &mut vals);
    if vals.is_empty() {
        return None;
    }
    fn chain(i: usize, vals: &[i32]) -> Option<Box<TreeNode>> {
        if i == vals.len() {
            return None;
        }
        Some(Box::new(TreeNode {
            val: vals[i],
            left: None,
            right: chain(i + 1, vals),
        }))
    }
    chain(0, &vals)
}

fn main() {
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
            val: 5,
            left: None,
            right: None,
        })),
    }));
    println!("{}", tree_to_doubly_list(root).unwrap().val);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vals(mut h: Option<Box<TreeNode>>) -> Vec<i32> {
        let mut out = vec![];
        while let Some(n) = h {
            out.push(n.val);
            h = n.right;
        }
        out
    }

    #[test]
    fn ordered_chain() {
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
                val: 5,
                left: None,
                right: None,
            })),
        }));
        assert_eq!(
            vals(tree_to_doubly_list(root)),
            vec![1, 2, 3, 4, 5]
        );
    }
}
