/// LeetCode #938 - Range Sum of BST
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<TreeNode>>,
    pub right: Option<Rc<TreeNode>>,
}

fn range_sum_bst(root: Option<Rc<TreeNode>>, low: i32, high: i32) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            let mut sum = 0;
            if n.val >= low && n.val <= high {
                sum += n.val;
            }
            if n.val > low {
                sum += range_sum_bst(n.left.clone(), low, high);
            }
            if n.val < high {
                sum += range_sum_bst(n.right.clone(), low, high);
            }
            sum
        }
    }
}

fn main() {
    let root = Rc::new(TreeNode {
        val: 10,
        left: Some(Rc::new(TreeNode {
            val: 5,
            left: Some(Rc::new(TreeNode { val: 3, left: None, right: None })),
            right: Some(Rc::new(TreeNode { val: 7, left: None, right: None })),
        })),
        right: Some(Rc::new(TreeNode {
            val: 15,
            left: None,
            right: Some(Rc::new(TreeNode { val: 18, left: None, right: None })),
        })),
    });
    println!("{}", range_sum_bst(Some(root), 7, 15));
}

#[cfg(test)]
mod tests {
    use super::{range_sum_bst, TreeNode};
    use std::rc::Rc;

    fn tree() -> Rc<TreeNode> {
        Rc::new(TreeNode {
            val: 10,
            left: Some(Rc::new(TreeNode {
                val: 5,
                left: Some(Rc::new(TreeNode { val: 3, left: None, right: None })),
                right: Some(Rc::new(TreeNode { val: 7, left: None, right: None })),
            })),
            right: Some(Rc::new(TreeNode {
                val: 15,
                left: None,
                right: Some(Rc::new(TreeNode { val: 18, left: None, right: None })),
            })),
        })
    }

    #[test]
    fn example_one() {
        assert_eq!(range_sum_bst(Some(tree()), 7, 15), 32);
    }

    #[test]
    fn example_two() {
        assert_eq!(range_sum_bst(Some(tree()), 3, 7), 15);
    }
}
