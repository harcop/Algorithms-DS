/// LeetCode #106 - Construct Binary Tree from Inorder and Postorder Traversal
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

fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Box<TreeNode>> {
    fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Box<TreeNode>> {
        if inorder.is_empty() {
            return None;
        }
        let root_val = postorder[postorder.len() - 1];
        let idx = inorder.iter().position(|&x| x == root_val).unwrap();
        let mut root = Box::new(TreeNode::new(root_val));
        root.left = helper(&inorder[..idx], &postorder[..idx]);
        root.right = helper(
            &inorder[idx + 1..],
            &postorder[idx..postorder.len() - 1],
        );
        Some(root)
    }
    helper(&inorder, &postorder)
}

fn main() {
    println!(
        "{:?}",
        build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
    );
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    #[test]
    fn example_one() {
        let root = build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);
        assert!(root.is_some());
        assert_eq!(root.unwrap().val, 3);
    }

    #[test]
    fn example_two() {
        assert!(build_tree(vec![-1], vec![-1]).is_some());
    }
}
