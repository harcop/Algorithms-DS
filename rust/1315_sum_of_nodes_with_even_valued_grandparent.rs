/// LeetCode #1315 - Sum of Nodes with Even-Valued Grandparent
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, p: i32, gp: i32, sum: &mut i32) {
        if let Some(n) = node {
            let n = n.borrow();
            if gp % 2 == 0 {
                *sum += n.val;
            }
            dfs(&n.left, n.val, p, sum);
            dfs(&n.right, n.val, p, sum);
        }
    }
    let mut sum = 0;
    dfs(&root, 1, 1, &mut sum);
    sum
}

fn main() {
    println!("{}", sum_even_grandparent(None));
}

#[cfg(test)]
mod tests {
    use super::{sum_even_grandparent, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example_one() {
        let root = Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None }))),
                right: None,
            }))),
            right: None,
        }));
        assert_eq!(sum_even_grandparent(Some(root)), 2);
    }
}
