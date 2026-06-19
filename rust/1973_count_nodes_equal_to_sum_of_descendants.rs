/// LeetCode #1973 - Count Nodes Equal to Sum of Descendants
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn equal_to_descendants(root: Option<Box<TreeNode>>) -> i32 {
    let mut ans = 0i32;

    fn dfs(node: Option<Box<TreeNode>>, ans: &mut i32) -> i32 {
        let Some(n) = node else {
            return 0;
        };
        let l = dfs(n.left, ans);
        let r = dfs(n.right, ans);
        if l + r == n.val {
            *ans += 1;
        }
        n.val + l + r
    }

    dfs(root, &mut ans);
    ans
}

fn main() {
    let mut root = Box::new(TreeNode {
        val: 10,
        left: None,
        right: None,
    });
    root.left = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 2,
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
        val: 4,
        left: None,
        right: None,
    }));
    println!("{}", equal_to_descendants(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{equal_to_descendants, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode {
            val: 10,
            left: None,
            right: None,
        });
        root.left = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 2,
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
            val: 4,
            left: None,
            right: None,
        }));
        assert_eq!(equal_to_descendants(Some(root)), 2);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        });
        root.left = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: None,
        }));
        assert_eq!(equal_to_descendants(Some(root)), 0);
    }

    #[test]
    fn example_three() {
        let root = Box::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        });
        assert_eq!(equal_to_descendants(Some(root)), 1);
    }
}
