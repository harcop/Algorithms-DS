/// LeetCode #2265 - Count Nodes Equal to Average of Subtree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn average_of_subtree(root: Option<Box<TreeNode>>) -> i32 {
    let mut ans = 0;

    fn dfs(node: Option<Box<TreeNode>>, ans: &mut i32) -> (i32, i32) {
        let Some(n) = node else {
            return (0, 0);
        };
        let (ls, lc) = dfs(n.left, ans);
        let (rs, rc) = dfs(n.right, ans);
        let sum = n.val + ls + rs;
        let count = lc + rc + 1;
        if sum / count == n.val {
            *ans += 1;
        }
        (sum, count)
    }

    dfs(root, &mut ans);
    ans
}

fn main() {
    let mut root = Box::new(TreeNode {
        val: 4,
        left: None,
        right: None,
    });
    root.left = Some(Box::new(TreeNode {
        val: 8,
        left: Some(Box::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })),
    }));
    root.right = Some(Box::new(TreeNode {
        val: 5,
        left: None,
        right: Some(Box::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        })),
    }));
    println!("{}", average_of_subtree(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{average_of_subtree, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        });
        root.left = Some(Box::new(TreeNode {
            val: 8,
            left: Some(Box::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
        }));
        root.right = Some(Box::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            })),
        }));
        assert_eq!(average_of_subtree(Some(root)), 5);
    }

    #[test]
    fn example_two() {
        let root = Box::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        });
        assert_eq!(average_of_subtree(Some(root)), 1);
    }
}
