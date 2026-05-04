/// LeetCode #298 - Binary Tree Longest Consecutive Sequence
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn longest_consecutive(root: Option<Box<TreeNode>>) -> i32 {
    let mut best = 0;
    fn dfs(n: &Option<Box<TreeNode>>, prev: Option<i32>, len: i32, best: &mut i32) {
        let Some(b) = n else {
            return;
        };
        let nl = match prev {
            Some(p) if b.val == p + 1 => len + 1,
            _ => 1,
        };
        *best = (*best).max(nl);
        dfs(&b.left, Some(b.val), nl, best);
        dfs(&b.right, Some(b.val), nl, best);
    }
    dfs(&root, None, 0, &mut best);
    best
}

fn main() {
    println!("{}", longest_consecutive(None));
}

#[cfg(test)]
mod tests {
    use super::{longest_consecutive, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Box::new(TreeNode {
                val: 3,
                left: Some(Box::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Box::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    })),
                })),
            })),
        });
        assert_eq!(longest_consecutive(Some(root)), 3);
    }
}
