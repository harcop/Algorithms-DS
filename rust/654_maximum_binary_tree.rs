/// LeetCode #654 - Maximum Binary Tree
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Box<TreeNode>> {
    fn build(nums: &[i32]) -> Option<Box<TreeNode>> {
        if nums.is_empty() { return None; }
        let (i, _) = nums.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
        Some(Box::new(TreeNode {
            val: nums[i],
            left: build(&nums[..i]),
            right: build(&nums[i + 1..]),
        }))
    }
    build(&nums)
}

fn main() {
    println!("{}", construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]).is_some());
}

#[cfg(test)]
mod tests {
    use super::construct_maximum_binary_tree;

    #[test]
    fn example_one() {
        let t = construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]).unwrap();
        assert_eq!(t.val, 6);
        assert_eq!(t.left.as_ref().unwrap().val, 3);
        assert_eq!(t.right.as_ref().unwrap().val, 5);
    }
}
