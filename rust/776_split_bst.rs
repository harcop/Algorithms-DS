/// LeetCode #776 - Split BST
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
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

fn split_bst(root: Option<Box<TreeNode>>, target: i32) -> Vec<Option<Box<TreeNode>>> {
    let (small, large) = split(root, target);
    vec![small, large]
}

fn split(root: Option<Box<TreeNode>>, target: i32) -> (Option<Box<TreeNode>>, Option<Box<TreeNode>>) {
    match root {
        None => (None, None),
        Some(mut node) => {
            if node.val <= target {
                let (s, l) = split(node.right.take(), target);
                node.right = s;
                (Some(node), l)
            } else {
                let (s, l) = split(node.left.take(), target);
                node.left = l;
                (s, Some(node))
            }
        }
    }
}

fn main() {
    let root = Some(Box::new(TreeNode::new(1)));
    println!("{:?}", split_bst(root, 1));
}

#[cfg(test)]
mod tests {
    use super::{split_bst, TreeNode};

    fn collect(root: &Option<Box<TreeNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        fn rec(node: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
            if let Some(n) = node {
                rec(&n.left, out);
                out.push(n.val);
                rec(&n.right, out);
            }
        }
        rec(root, &mut out);
        out
    }

    #[test]
    fn example_one() {
        let mut n4 = Box::new(TreeNode::new(4));
        let mut n2 = Box::new(TreeNode::new(2));
        n2.left = Some(Box::new(TreeNode::new(1)));
        n2.right = Some(Box::new(TreeNode::new(3)));
        let mut n6 = Box::new(TreeNode::new(6));
        n6.left = Some(Box::new(TreeNode::new(5)));
        n6.right = Some(Box::new(TreeNode::new(7)));
        n4.left = Some(n2);
        n4.right = Some(n6);
        let parts = split_bst(Some(n4), 2);
        assert_eq!(collect(&parts[0]), vec![1, 2]);
        assert_eq!(collect(&parts[1]), vec![3, 4, 5, 6, 7]);
    }

    #[test]
    fn example_two() {
        let parts = split_bst(Some(Box::new(TreeNode::new(1))), 1);
        assert_eq!(collect(&parts[0]), vec![1]);
        assert!(parts[1].is_none());
    }
}
