/// LeetCode #3319 - K-th Largest Perfect Subtree Size in Binary Tree
#[derive(Debug, Clone, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn leaf(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn kth_largest_perfect_subtree(root: Option<Box<TreeNode>>, k: i32) -> i32 {
    let mut nums = Vec::new();
    fn dfs(root: &Option<Box<TreeNode>>, nums: &mut Vec<i32>) -> i32 {
        let Some(node) = root else {
            return 0;
        };
        let l = dfs(&node.left, nums);
        let r = dfs(&node.right, nums);
        if l < 0 || l != r {
            return -1;
        }
        let cnt = l + r + 1;
        nums.push(cnt);
        cnt
    }
    dfs(&root, &mut nums);
    if nums.len() < k as usize {
        return -1;
    }
    nums.sort_by(|a, b| b.cmp(a));
    nums[k as usize - 1]
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode::leaf(2))),
        right: Some(Box::new(TreeNode::leaf(3))),
    }));
    println!("{}", kth_largest_perfect_subtree(root, 1));
}

#[cfg(test)]
mod tests {
    use super::{kth_largest_perfect_subtree, TreeNode};

    #[test]
    fn example1() {
        // [5,3,6,5,2,5,7,1,8,null,null,6,8]
        let root = Some(Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode {
                val: 3,
                left: Some(Box::new(TreeNode {
                    val: 5,
                    left: Some(Box::new(TreeNode::leaf(1))),
                    right: Some(Box::new(TreeNode::leaf(8))),
                })),
                right: Some(Box::new(TreeNode::leaf(2))),
            })),
            right: Some(Box::new(TreeNode {
                val: 6,
                left: Some(Box::new(TreeNode {
                    val: 5,
                    left: Some(Box::new(TreeNode::leaf(6))),
                    right: Some(Box::new(TreeNode::leaf(8))),
                })),
                right: Some(Box::new(TreeNode::leaf(7))),
            })),
        }));
        assert_eq!(kth_largest_perfect_subtree(root, 2), 3);
    }

    #[test]
    fn example2() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode::leaf(4))),
                right: Some(Box::new(TreeNode::leaf(5))),
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: Some(Box::new(TreeNode::leaf(6))),
                right: Some(Box::new(TreeNode::leaf(7))),
            })),
        }));
        assert_eq!(kth_largest_perfect_subtree(root, 1), 7);
    }

    #[test]
    fn example3() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Box::new(TreeNode::leaf(4))),
            })),
            right: Some(Box::new(TreeNode::leaf(3))),
        }));
        assert_eq!(kth_largest_perfect_subtree(root, 3), -1);
    }
}
