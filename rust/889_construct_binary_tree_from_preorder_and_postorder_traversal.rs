/// LeetCode #889 - Construct Binary Tree from Preorder and Postorder Traversal
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

fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Box<TreeNode>> {
    fn build(pre: &[i32], post: &[i32]) -> Option<Box<TreeNode>> {
        if pre.is_empty() {
            return None;
        }
        let v = pre[0];
        let mut root = Box::new(TreeNode::new(v));
        if pre.len() == 1 {
            return Some(root);
        }
        let left_root = pre[1];
        let left_size = post.iter().position(|&x| x == left_root).unwrap() + 1;
        root.left = build(&pre[1..left_size + 1], &post[..left_size]);
        root.right = build(&pre[left_size + 1..], &post[left_size..post.len() - 1]);
        Some(root)
    }
    build(&preorder, &postorder)
}

fn main() {
    println!(
        "{:?}",
        construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::construct_from_pre_post;

    #[test]
    fn example_one() {
        assert!(construct_from_pre_post(
            vec![1, 2, 4, 5, 3, 6, 7],
            vec![4, 5, 2, 6, 7, 3, 1]
        )
        .is_some());
    }
}
