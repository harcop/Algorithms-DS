/// LeetCode #222 - Count Complete Tree Nodes
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn count_nodes(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(root: Option<Box<TreeNode>>) -> i32 {
        let Some(r) = root else {
            return 0;
        };
        1 + dfs(r.left) + dfs(r.right)
    }
    dfs(root)
}

fn main() {
    println!("{}", count_nodes(None));
}

#[cfg(test)]
mod tests {
    use super::{count_nodes, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
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
            right: Some(Box::new(TreeNode {
                val: 3,
                left: Some(Box::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        });
        assert_eq!(count_nodes(Some(root)), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_nodes(None), 0);
    }

    #[test]
    fn example_three() {
        let root = Box::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        });
        assert_eq!(count_nodes(Some(root)), 1);
    }
}
