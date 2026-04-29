/// LeetCode #108 - Convert Sorted Array to Binary Search Tree
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

fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Box<TreeNode>> {
    fn build(nums: &[i32]) -> Option<Box<TreeNode>> {
        if nums.is_empty() {
            return None;
        }
        let mid = nums.len() / 2;
        let mut root = Box::new(TreeNode::new(nums[mid]));
        root.left = build(&nums[..mid]);
        root.right = build(&nums[mid + 1..]);
        Some(root)
    }
    build(&nums)
}

fn main() {
    println!("{:?}", sorted_array_to_bst(vec![-10, -3, 0, 5, 9]));
}

#[cfg(test)]
mod tests {
    use super::sorted_array_to_bst;

    #[test]
    fn example_one() {
        let root = sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
        assert!(root.is_some());
        assert_eq!(root.unwrap().val, 0);
    }

    #[test]
    fn example_two() {
        assert!(sorted_array_to_bst(vec![1, 3]).is_some());
    }
}
