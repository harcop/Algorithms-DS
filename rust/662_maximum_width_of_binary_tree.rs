/// LeetCode #662 - Maximum Width of Binary Tree
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

fn width_of_binary_tree(root: Option<Box<TreeNode>>) -> i32 {
    let Some(root) = root else {
        return 0;
    };
    let mut ans = 1i64;
    let mut q = vec![(root, 0u64)];
    while !q.is_empty() {
        let start = q[0].1;
        let end = q.last().unwrap().1;
        ans = ans.max((end - start + 1) as i64);
        let mut nq = vec![];
        for (node, idx) in q {
            let rel = idx - start;
            if let Some(l) = node.left {
                nq.push((l, rel * 2));
            }
            if let Some(r) = node.right {
                nq.push((r, rel * 2 + 1));
            }
        }
        q = nq;
    }
    ans as i32
}

fn main() {
    let mut root = Box::new(TreeNode::new(1));
    root.left = Some(Box::new(TreeNode::new(3)));
    root.right = Some(Box::new(TreeNode::new(2)));
    println!("{}", width_of_binary_tree(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{width_of_binary_tree, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(1));
        let mut l = Box::new(TreeNode::new(3));
        l.left = Some(Box::new(TreeNode::new(5)));
        l.right = Some(Box::new(TreeNode::new(3)));
        let mut r = Box::new(TreeNode::new(2));
        r.right = Some(Box::new(TreeNode::new(9)));
        root.left = Some(l);
        root.right = Some(r);
        assert_eq!(width_of_binary_tree(Some(root)), 4);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        let mut l = Box::new(TreeNode::new(3));
        let mut ll = Box::new(TreeNode::new(5));
        ll.left = Some(Box::new(TreeNode::new(6)));
        l.left = Some(ll);
        let mut r = Box::new(TreeNode::new(2));
        let mut rr = Box::new(TreeNode::new(9));
        rr.left = Some(Box::new(TreeNode::new(7)));
        r.right = Some(rr);
        root.left = Some(l);
        root.right = Some(r);
        assert_eq!(width_of_binary_tree(Some(root)), 7);
    }

    #[test]
    fn example_three() {
        let mut root = Box::new(TreeNode::new(1));
        let mut l = Box::new(TreeNode::new(3));
        l.left = Some(Box::new(TreeNode::new(5)));
        root.left = Some(l);
        root.right = Some(Box::new(TreeNode::new(2)));
        assert_eq!(width_of_binary_tree(Some(root)), 2);
    }
}
