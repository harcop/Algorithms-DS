/// LeetCode #545 - Boundary of Binary Tree
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

fn is_leaf(n: &TreeNode) -> bool {
    n.left.is_none() && n.right.is_none()
}

fn boundary_of_binary_tree(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let Some(root) = root else {
        return vec![];
    };
    let mut ans = vec![root.val];
    if is_leaf(&root) {
        return ans;
    }
    fn left_bound(node: &Option<Box<TreeNode>>, ans: &mut Vec<i32>) {
        if let Some(n) = node {
            if is_leaf(n) {
                return;
            }
            ans.push(n.val);
            if n.left.is_some() {
                left_bound(&n.left, ans);
            } else {
                left_bound(&n.right, ans);
            }
        }
    }
    fn leaves(node: &Option<Box<TreeNode>>, ans: &mut Vec<i32>) {
        if let Some(n) = node {
            if is_leaf(n) {
                ans.push(n.val);
                return;
            }
            leaves(&n.left, ans);
            leaves(&n.right, ans);
        }
    }
    fn right_bound(node: &Option<Box<TreeNode>>, ans: &mut Vec<i32>) {
        if let Some(n) = node {
            if is_leaf(n) {
                return;
            }
            if n.right.is_some() {
                right_bound(&n.right, ans);
            } else {
                right_bound(&n.left, ans);
            }
            ans.push(n.val);
        }
    }
    left_bound(&root.left, &mut ans);
    leaves(&Some(root.clone()), &mut ans);
    right_bound(&root.right, &mut ans);
    ans
}

fn main() {
    let mut root = Box::new(TreeNode::new(1));
    let mut r = Box::new(TreeNode::new(2));
    r.left = Some(Box::new(TreeNode::new(3)));
    r.right = Some(Box::new(TreeNode::new(4)));
    root.right = Some(r);
    println!("{:?}", boundary_of_binary_tree(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{boundary_of_binary_tree, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(1));
        let mut r = Box::new(TreeNode::new(2));
        r.left = Some(Box::new(TreeNode::new(3)));
        r.right = Some(Box::new(TreeNode::new(4)));
        root.right = Some(r);
        assert_eq!(boundary_of_binary_tree(Some(root)), vec![1, 3, 4, 2]);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        let mut l = Box::new(TreeNode::new(2));
        l.left = Some(Box::new(TreeNode::new(4)));
        let mut l5 = Box::new(TreeNode::new(5));
        l5.left = Some(Box::new(TreeNode::new(7)));
        l5.right = Some(Box::new(TreeNode::new(8)));
        l.right = Some(l5);
        let mut r = Box::new(TreeNode::new(3));
        let mut r6 = Box::new(TreeNode::new(6));
        r6.left = Some(Box::new(TreeNode::new(9)));
        r6.right = Some(Box::new(TreeNode::new(10)));
        r.left = Some(r6);
        root.left = Some(l);
        root.right = Some(r);
        assert_eq!(
            boundary_of_binary_tree(Some(root)),
            vec![1, 2, 4, 7, 8, 9, 10, 6, 3]
        );
    }
}
