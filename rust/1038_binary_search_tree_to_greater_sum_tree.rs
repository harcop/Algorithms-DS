/// LeetCode #1038 - Binary Search Tree to Greater Sum Tree
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn bst_to_gst(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    fn dfs(node: Option<Box<TreeNode>>, acc: &mut i32) -> Option<Box<TreeNode>> {
        let mut node = node?;
        let right = dfs(node.right.take(), acc);
        *acc += node.val;
        node.val = *acc;
        let left = dfs(node.left.take(), acc);
        node.right = right;
        node.left = left;
        Some(node)
    }
    dfs(root, &mut 0)
}

fn main() {
    let _ = bst_to_gst(Some(Box::new(TreeNode {
        val: 4,
        left: Some(Box::new(TreeNode { val: 1, left: None, right: None })),
        right: Some(Box::new(TreeNode { val: 6, left: None, right: None })),
    })));
}

#[cfg(test)]
mod tests {
    use super::{bst_to_gst, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 4,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: Some(Box::new(TreeNode { val: 0, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
            })),
            right: Some(Box::new(TreeNode {
                val: 6,
                left: Some(Box::new(TreeNode { val: 5, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 7, left: None, right: None })),
            })),
        }));
        assert_eq!(bst_to_gst(root).unwrap().val, 22);
    }
}
