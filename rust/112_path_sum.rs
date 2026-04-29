/// LeetCode #112 - Path Sum
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

fn has_path_sum(root: Option<Box<TreeNode>>, target_sum: i32) -> bool {
    fn dfs(node: &Option<Box<TreeNode>>, remain: i32) -> bool {
        match node {
            None => false,
            Some(n) => {
                let remain = remain - n.val;
                if n.left.is_none() && n.right.is_none() {
                    return remain == 0;
                }
                dfs(&n.left, remain) || dfs(&n.right, remain)
            }
        }
    }
    dfs(&root, target_sum)
}

fn main() {
    let mut root = Box::new(TreeNode::new(5));
    root.left = Some(Box::new(TreeNode::new(4)));
    root.right = Some(Box::new(TreeNode::new(8)));
    println!("{}", has_path_sum(Some(root), 9));
}

#[cfg(test)]
mod tests {
    use super::{has_path_sum, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(5));
        root.left = Some(Box::new(TreeNode::new(4)));
        root.right = Some(Box::new(TreeNode::new(8)));
        root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(11)));
        root.left.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(7)));
        root.left.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(2)));
        root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(13)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(4)));
        root.right.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(1)));
        assert!(has_path_sum(Some(root), 22));
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(3)));
        assert!(!has_path_sum(Some(root), 5));
    }
}
