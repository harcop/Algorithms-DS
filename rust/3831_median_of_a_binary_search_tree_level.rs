/// LeetCode #3831 - Median of a Binary Search Tree Level (premium)
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn level_median(root: Option<Box<TreeNode>>, level: i32) -> i32 {
    let mut nums = Vec::new();
    fn dfs(node: &Option<Box<TreeNode>>, i: i32, level: i32, nums: &mut Vec<i32>) {
        let Some(node) = node else {
            return;
        };
        dfs(&node.left, i + 1, level, nums);
        if i == level {
            nums.push(node.val);
        }
        dfs(&node.right, i + 1, level, nums);
    }
    dfs(&root, 0, level, &mut nums);
    if nums.is_empty() {
        -1
    } else {
        nums[nums.len() / 2]
    }
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 4,
        left: None,
        right: Some(Box::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
        })),
    }));
    println!("{}", level_median(root, 2));
}

#[cfg(test)]
mod tests {
    use super::{level_median, TreeNode};

    #[test]
    fn example1() {
        let root = Some(Box::new(TreeNode {
            val: 4,
            left: None,
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
            })),
        }));
        assert_eq!(level_median(root, 2), 7);
    }

    #[test]
    fn example2() {
        let root = Some(Box::new(TreeNode {
            val: 6,
            left: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 8,
                left: None,
                right: None,
            })),
        }));
        assert_eq!(level_median(root, 1), 8);
    }

    #[test]
    fn example3() {
        let root = Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
            right: None,
        }));
        assert_eq!(level_median(root, 2), -1);
    }
}
