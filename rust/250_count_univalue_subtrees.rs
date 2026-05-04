/// LeetCode #250 - Count Univalue Subtrees
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn count_unival_subtrees(root: Option<Box<TreeNode>>) -> i32 {
    let mut ans = 0;
    fn dfs(node: &Option<Box<TreeNode>>, ans: &mut i32) -> bool {
        let Some(n) = node else {
            return true;
        };
        let l_ok = dfs(&n.left, ans);
        let r_ok = dfs(&n.right, ans);
        let mut uni = l_ok && r_ok;
        if let Some(l) = &n.left {
            if l.val != n.val {
                uni = false;
            }
        }
        if let Some(r) = &n.right {
            if r.val != n.val {
                uni = false;
            }
        }
        if uni {
            *ans += 1;
        }
        uni
    }
    dfs(&root, &mut ans);
    ans
}

fn main() {
    println!("{}", count_unival_subtrees(None));
}

#[cfg(test)]
mod tests {
    use super::{count_unival_subtrees, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: Some(Box::new(TreeNode {
                    val: 5,
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
                val: 5,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                })),
            })),
        });
        assert_eq!(count_unival_subtrees(Some(root)), 4);
    }
}
