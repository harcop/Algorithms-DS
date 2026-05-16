/// LeetCode #865 - Smallest Subtree with all the Deepest Nodes
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<TreeNode>>,
    pub right: Option<Rc<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn subtree_with_all_deepest(root: Option<Rc<TreeNode>>) -> Option<Rc<TreeNode>> {
    fn dfs(node: Option<Rc<TreeNode>>) -> (i32, Option<Rc<TreeNode>>) {
        match node {
            None => (-1, None),
            Some(n) => {
                let (dl, left) = dfs(n.left.clone());
                let (dr, right) = dfs(n.right.clone());
                if dl == dr {
                    (dl + 1, Some(n))
                } else if dl > dr {
                    (dl + 1, left)
                } else {
                    (dr + 1, right)
                }
            }
        }
    }
    dfs(root).1
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::{subtree_with_all_deepest, TreeNode};
    use std::rc::Rc;

    #[test]
    fn example_one() {
        let mut two = Rc::new(TreeNode::new(2));
        two.left = Some(Rc::new(TreeNode::new(7)));
        two.right = Some(Rc::new(TreeNode::new(4)));
        let mut five = Rc::new(TreeNode::new(5));
        five.left = Some(Rc::new(TreeNode::new(6)));
        five.right = Some(two);
        let mut root = Rc::new(TreeNode::new(3));
        root.left = Some(five);
        root.right = Some(Rc::new(TreeNode::new(1)));
        let ans = subtree_with_all_deepest(Some(root));
        assert_eq!(ans.unwrap().val, 2);
    }
}
