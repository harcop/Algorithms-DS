/// LeetCode #951 - Flip Equivalent Binary Trees
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<TreeNode>>,
    pub right: Option<Rc<TreeNode>>,
}

fn flip_equiv(root1: Option<Rc<TreeNode>>, root2: Option<Rc<TreeNode>>) -> bool {
    match (&root1, &root2) {
        (None, None) => true,
        (Some(a), Some(b)) if a.val == b.val => {
            flip_equiv(a.left.clone(), b.left.clone()) && flip_equiv(a.right.clone(), b.right.clone())
                || flip_equiv(a.left.clone(), b.right.clone()) && flip_equiv(a.right.clone(), b.left.clone())
        }
        _ => false,
    }
}

fn main() {
    println!("{}", flip_equiv(None, None));
}

#[cfg(test)]
mod tests {
    use super::{flip_equiv, TreeNode};
    use std::rc::Rc;

    fn node(val: i32, l: Option<Rc<TreeNode>>, r: Option<Rc<TreeNode>>) -> Rc<TreeNode> {
        Rc::new(TreeNode { val, left: l, right: r })
    }

    #[test]
    fn example_one() {
        let t1 = node(
            1,
            Some(node(2, None, None)),
            Some(node(3, Some(node(4, None, None)), Some(node(5, Some(node(7, None, None)), Some(node(8, None, None)))))),
        );
        let t2 = node(
            1,
            Some(node(3, Some(node(5, Some(node(7, None, None)), Some(node(8, None, None)))), Some(node(4, None, None)))),
            Some(node(2, None, None)),
        );
        assert!(flip_equiv(Some(t1), Some(t2)));
    }

    #[test]
    fn example_two() {
        assert!(!flip_equiv(None, Some(Rc::new(TreeNode { val: 1, left: None, right: None }))));
    }
}
