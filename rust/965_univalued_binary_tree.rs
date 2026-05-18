/// LeetCode #965 - Univalued Binary Tree
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<TreeNode>>,
    pub right: Option<Rc<TreeNode>>,
}

fn is_univalued_tree(root: Option<Rc<TreeNode>>) -> bool {
    let Some(root) = root else {
        return true;
    };
    let val = root.val;
    fn dfs(node: Option<Rc<TreeNode>>, val: i32) -> bool {
        match node {
            None => true,
            Some(n) => n.val == val && dfs(n.left.clone(), val) && dfs(n.right.clone(), val),
        }
    }
    dfs(Some(root), val)
}

fn main() {
    let root = Rc::new(TreeNode {
        val: 1,
        left: Some(Rc::new(TreeNode { val: 1, left: None, right: None })),
        right: Some(Rc::new(TreeNode { val: 1, left: None, right: None })),
    });
    println!("{}", is_univalued_tree(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{is_univalued_tree, TreeNode};
    use std::rc::Rc;

    #[test]
    fn example_one() {
        let root = Rc::new(TreeNode {
            val: 1,
            left: Some(Rc::new(TreeNode { val: 1, left: None, right: None })),
            right: Some(Rc::new(TreeNode { val: 1, left: None, right: None })),
        });
        assert!(is_univalued_tree(Some(root)));
    }

    #[test]
    fn example_two() {
        let root = Rc::new(TreeNode {
            val: 2,
            left: Some(Rc::new(TreeNode { val: 2, left: None, right: None })),
            right: Some(Rc::new(TreeNode { val: 5, left: None, right: None })),
        });
        assert!(!is_univalued_tree(Some(root)));
    }
}
