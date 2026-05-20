/// LeetCode #1160 - Distribute Coins in Binary Tree
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn distribute_coins(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(node: &Option<Box<TreeNode>>) -> (i32, i32) {
        match node {
            None => (0, 0),
            Some(n) => {
                let (lb, lm) = dfs(&n.left);
                let (rb, rm) = dfs(&n.right);
                let moves = lm + rm + lb.abs() + rb.abs();
                (n.val - 1 + lb + rb, moves)
            }
        }
    }
    dfs(&root).1
}

fn main() {
    println!("{}", distribute_coins(None));
}

#[cfg(test)]
mod tests {
    use super::{distribute_coins, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            })),
        });
        assert_eq!(distribute_coins(Some(root)), 2);
    }

    #[test]
    fn example_two() {
        let root = Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
        });
        assert_eq!(distribute_coins(Some(root)), 2);
    }
}
