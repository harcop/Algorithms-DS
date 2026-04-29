use std::collections::HashMap;

/// LeetCode #105 - Construct Binary Tree from Preorder and Inorder Traversal
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

fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Box<TreeNode>> {
    let mut index_map: HashMap<i32, usize> = HashMap::new();
    for (i, &v) in inorder.iter().enumerate() {
        index_map.insert(v, i);
    }

    fn helper(
        preorder: &[i32],
        pre_lo: usize,
        pre_hi: usize,
        in_lo: usize,
        in_hi: usize,
        index_map: &HashMap<i32, usize>,
    ) -> Option<Box<TreeNode>> {
        if pre_lo > pre_hi || in_lo > in_hi {
            return None;
        }
        let root_val = preorder[pre_lo];
        let idx = *index_map.get(&root_val).unwrap();
        let left_size = idx.saturating_sub(in_lo);

        let mut root = Box::new(TreeNode::new(root_val));
        root.left = helper(
            preorder,
            pre_lo + 1,
            pre_lo + left_size,
            in_lo,
            idx.saturating_sub(1),
            index_map,
        );
        root.right = helper(
            preorder,
            pre_lo + left_size + 1,
            pre_hi,
            idx + 1,
            in_hi,
            index_map,
        );
        Some(root)
    }

    let n = preorder.len();
    if n == 0 {
        return None;
    }
    helper(&preorder, 0, n - 1, 0, n - 1, &index_map)
}

fn main() {
    println!(
        "{:?}",
        build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
    );
}

#[cfg(test)]
mod tests {
    use super::build_tree;

    #[test]
    fn example_one() {
        let root = build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        assert!(root.is_some());
        let r = root.unwrap();
        assert_eq!(r.val, 3);
        assert_eq!(r.left.as_ref().unwrap().val, 9);
        assert_eq!(r.right.as_ref().unwrap().val, 20);
    }

    #[test]
    fn example_two() {
        assert!(build_tree(vec![-1], vec![-1]).is_some());
    }
}
