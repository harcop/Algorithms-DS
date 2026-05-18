/// LeetCode #872 - Leaf-Similar Trees
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<TreeNode>>,
    pub right: Option<Rc<TreeNode>>,
}

fn leaves(root: Option<Rc<TreeNode>>, out: &mut Vec<i32>) {
    match root {
        None => {}
        Some(n) => {
            if n.left.is_none() && n.right.is_none() {
                out.push(n.val);
            } else {
                leaves(n.left.clone(), out);
                leaves(n.right.clone(), out);
            }
        }
    }
}

fn leaf_similar(root1: Option<Rc<TreeNode>>, root2: Option<Rc<TreeNode>>) -> bool {
    let mut a = vec![];
    let mut b = vec![];
    leaves(root1, &mut a);
    leaves(root2, &mut b);
    a == b
}

fn main() {
    println!("{}", leaf_similar(None, None));
}

#[cfg(test)]
mod tests {
    use super::{leaf_similar, TreeNode};
    use std::rc::Rc;

    fn node(val: i32, left: Option<Rc<TreeNode>>, right: Option<Rc<TreeNode>>) -> Rc<TreeNode> {
        Rc::new(TreeNode { val, left, right })
    }

    #[test]
    fn example_one() {
        let t1_l = node(
            5,
            Some(node(6, None, None)),
            Some(node(
                2,
                Some(node(7, None, None)),
                Some(node(4, None, None)),
            )),
        );
        let t1_r = node(
            1,
            Some(node(9, None, None)),
            Some(node(8, None, None)),
        );
        let t1 = node(3, Some(t1_l), Some(t1_r));

        let t2_l = node(
            5,
            Some(node(6, None, None)),
            Some(node(7, None, None)),
        );
        let t2_r = node(
            1,
            Some(node(4, None, None)),
            Some(node(
                2,
                Some(node(9, None, None)),
                Some(node(8, None, None)),
            )),
        );
        let t2 = node(3, Some(t2_l), Some(t2_r));
        assert!(leaf_similar(Some(t1), Some(t2)));
    }
}
