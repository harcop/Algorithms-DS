/// LeetCode #2773 - Height of Special Binary Tree
#[derive(Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn height_of_tree(root: Option<Box<TreeNode>>) -> i32 {
    let mut ans = 0;
    fn dfs(node: &TreeNode, d: i32, ans: &mut i32) {
        *ans = (*ans).max(d);
        let nd = d + 1;
        if let Some(left) = &node.left {
            let back = left
                .right
                .as_deref()
                .map(|n| std::ptr::eq(n, node))
                .unwrap_or(false);
            if !back {
                dfs(left, nd, ans);
            }
        }
        if let Some(right) = &node.right {
            let back = right
                .left
                .as_deref()
                .map(|n| std::ptr::eq(n, node))
                .unwrap_or(false);
            if !back {
                dfs(right, nd, ans);
            }
        }
    }
    if let Some(root) = &root {
        dfs(root, 0, &mut ans);
    }
    ans
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            })),
        })),
    }));
    println!("{}", height_of_tree(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: Some(Box::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                })),
            })),
        }));
        assert_eq!(height_of_tree(root), 2);
    }

    #[test]
    fn example_two() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: None,
        }));
        assert_eq!(height_of_tree(root), 1);
    }
}
