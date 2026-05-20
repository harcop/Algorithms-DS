/// LeetCode #1145 - Binary Tree Coloring Game
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn btree_game_winning_move(root: Option<Box<TreeNode>>, n: i32, x: i32) -> bool {
    let mut left = 0i32;
    let mut right = 0i32;
    let mut parent = 0i32;
    fn dfs(
        node: &Option<Box<TreeNode>>,
        x: i32,
        n: i32,
        left: &mut i32,
        right: &mut i32,
        parent: &mut i32,
    ) -> i32 {
        match node {
            None => 0,
            Some(nod) => {
                let l = dfs(&nod.left, x, n, left, right, parent);
                let r = dfs(&nod.right, x, n, left, right, parent);
                if nod.val == x {
                    *left = l;
                    *right = r;
                    *parent = n - l - r - 1;
                }
                1 + l + r
            }
        }
    }
    dfs(&root, x, n, &mut left, &mut right, &mut parent);
    let blue = left.max(right).max(parent + 1);
    blue > n - blue
}

fn main() {
    println!("{}", btree_game_winning_move(None, 1, 1));
}

#[cfg(test)]
mod tests {
    use super::{btree_game_winning_move, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode {
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
        });
        assert!(btree_game_winning_move(Some(root), 5, 3));
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: None,
        });
        assert!(!btree_game_winning_move(Some(root), 2, 1));
    }
}
