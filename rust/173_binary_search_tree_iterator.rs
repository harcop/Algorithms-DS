/// LeetCode #173 - Binary Search Tree Iterator
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub struct BSTIterator {
    stack: Vec<TreeNode>,
}

impl BSTIterator {
    fn new(root: Option<Box<TreeNode>>) -> Self {
        let mut it = BSTIterator { stack: vec![] };
        it.push_left(root);
        it
    }

    fn push_left(&mut self, mut root: Option<Box<TreeNode>>) {
        while let Some(mut n) = root {
            root = n.left.take();
            self.stack.push(*n);
        }
    }

    fn next(&mut self) -> i32 {
        let mut node = self.stack.pop().unwrap();
        let v = node.val;
        self.push_left(node.right.take());
        v
    }

    fn has_next(&mut self) -> bool {
        !self.stack.is_empty()
    }
}

fn main() {
    let root = Box::new(TreeNode {
        val: 7,
        left: Some(Box::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 15,
            left: Some(Box::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 20,
                left: None,
                right: None,
            })),
        })),
    });
    let mut it = BSTIterator::new(Some(root));
    println!("{}", it.next());
}

#[cfg(test)]
mod tests {
    use super::{BSTIterator, TreeNode};

    #[test]
    fn example() {
        let root = Box::new(TreeNode {
            val: 7,
            left: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 15,
                left: Some(Box::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 20,
                    left: None,
                    right: None,
                })),
            })),
        });
        let mut it = BSTIterator::new(Some(root));
        assert_eq!(it.next(), 3);
        assert_eq!(it.next(), 7);
        assert!(it.has_next());
        assert_eq!(it.next(), 9);
        assert!(it.has_next());
        assert_eq!(it.next(), 15);
        assert!(it.has_next());
        assert_eq!(it.next(), 20);
        assert!(!it.has_next());
    }
}
