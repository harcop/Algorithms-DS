/// LeetCode #129 - Sum Root to Leaf Numbers
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

fn sum_numbers(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(node: &Option<Box<TreeNode>>, cur: i32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let v = cur * 10 + n.val;
                if n.left.is_none() && n.right.is_none() {
                    return v;
                }
                dfs(&n.left, v) + dfs(&n.right, v)
            }
        }
    }
    dfs(&root, 0)
}

fn main() {
    let mut r = Box::new(TreeNode::new(1));
    r.left = Some(Box::new(TreeNode::new(2)));
    r.right = Some(Box::new(TreeNode::new(3)));
    println!("{}", sum_numbers(Some(r)));
}

#[cfg(test)]
mod tests {
    use super::{sum_numbers, TreeNode};

    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode::new(1));
        r.left = Some(Box::new(TreeNode::new(2)));
        r.right = Some(Box::new(TreeNode::new(3)));
        assert_eq!(sum_numbers(Some(r)), 25);
    }

    #[test]
    fn example_two() {
        let mut r = Box::new(TreeNode::new(4));
        r.left = Some(Box::new(TreeNode::new(9)));
        r.right = Some(Box::new(TreeNode::new(0)));
        r.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(5)));
        r.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(1)));
        assert_eq!(sum_numbers(Some(r)), 1026);
    }
}
