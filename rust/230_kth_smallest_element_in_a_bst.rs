/// LeetCode #230 - Kth Smallest Element in a BST
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn kth_smallest(root: Option<Box<TreeNode>>, k: i32) -> i32 {
    let mut stack = vec![];
    let mut cur = root;
    let mut k = k;
    loop {
        while let Some(mut n) = cur {
            cur = n.left.take();
            stack.push(n);
        }
        let mut n = stack.pop().unwrap();
        k -= 1;
        if k == 0 {
            return n.val;
        }
        cur = n.right.take();
    }
}

fn main() {
    println!("{}", kth_smallest(None, 1));
}

#[cfg(test)]
mod tests {
    use super::{kth_smallest, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
        });
        assert_eq!(kth_smallest(Some(root), 1), 1);
    }

    #[test]
    fn example_two() {
        let root = Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode {
                val: 3,
                left: Some(Box::new(TreeNode {
                    val: 2,
                    left: Some(Box::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 4,
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
        assert_eq!(kth_smallest(Some(root), 3), 3);
    }
}
