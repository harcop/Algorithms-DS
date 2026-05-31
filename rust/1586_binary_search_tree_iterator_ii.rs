/// LeetCode #1586 - Binary Search Tree Iterator Ii
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub struct BSTIterator {
    vals: Vec<i32>,
    cur: i32,
}

impl BSTIterator {
    fn new(root: Option<Box<TreeNode>>) -> Self {
        let mut vals = vec![];
        fn dfs(n: Option<Box<TreeNode>>, out: &mut Vec<i32>) {
            if let Some(node) = n {
                dfs(node.left, out);
                out.push(node.val);
                dfs(node.right, out);
            }
        }
        dfs(root, &mut vals);
        BSTIterator { vals, cur: -1 }
    }
    fn has_next(&self) -> bool { (self.cur as usize + 1) < self.vals.len() }
    fn next(&mut self) -> i32 {
        self.cur += 1;
        self.vals[self.cur as usize]
    }
    fn has_prev(&self) -> bool { self.cur > 0 }
    fn prev(&mut self) -> i32 {
        self.cur -= 1;
        self.vals[self.cur as usize]
    }
}
fn main() {
    let root = Some(Box::new(TreeNode {
        val: 7,
        left: Some(Box::new(TreeNode { val: 3, left: None, right: None })),
        right: Some(Box::new(TreeNode {
            val: 15,
            left: Some(Box::new(TreeNode { val: 9, left: None, right: None })),
            right: Some(Box::new(TreeNode { val: 20, left: None, right: None })),
        })),
    }));
    let mut it = BSTIterator::new(root);
    println!("{}", it.next());
}
#[cfg(test)]
mod tests {
    use super::{BSTIterator, TreeNode};
    fn tree() -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode {
            val: 7,
            left: Some(Box::new(TreeNode { val: 3, left: None, right: None })),
            right: Some(Box::new(TreeNode {
                val: 15,
                left: Some(Box::new(TreeNode { val: 9, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 20, left: None, right: None })),
            })),
        }))
    }
    #[test]
    fn example() {
        let mut it = BSTIterator::new(tree());
        assert_eq!(it.next(), 3);
        assert_eq!(it.next(), 7);
        assert!(it.has_next());
        assert_eq!(it.next(), 9);
        assert_eq!(it.prev(), 7);
        assert_eq!(it.prev(), 3);
        assert!(!it.has_prev());
        assert_eq!(it.next(), 7);
        assert_eq!(it.next(), 9);
        assert_eq!(it.next(), 15);
        assert_eq!(it.next(), 20);
        assert!(!it.has_next());
    }
}
