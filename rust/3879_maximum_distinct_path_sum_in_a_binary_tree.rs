/// LeetCode #3879 - Maximum Distinct Path Sum in a Binary Tree (premium)
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn max_sum(root: Option<Box<TreeNode>>) -> i32 {
    let mut vals = Vec::new();
    let mut adj: Vec<Vec<usize>> = Vec::new();
    fn build(
        node: &Option<Box<TreeNode>>,
        parent: Option<usize>,
        vals: &mut Vec<i32>,
        adj: &mut Vec<Vec<usize>>,
    ) {
        let Some(node) = node else {
            return;
        };
        let u = vals.len();
        vals.push(node.val);
        adj.push(Vec::new());
        if let Some(p) = parent {
            adj[p].push(u);
            adj[u].push(p);
        }
        build(&node.left, Some(u), vals, adj);
        build(&node.right, Some(u), vals, adj);
    }
    build(&root, None, &mut vals, &mut adj);
    fn dfs2(u: usize, vals: &[i32], adj: &[Vec<usize>], vis: &mut HashSet<i32>) -> i32 {
        if vis.contains(&vals[u]) {
            return 0;
        }
        vis.insert(vals[u]);
        let mut best = 0;
        for &v in &adj[u] {
            best = best.max(dfs2(v, vals, adj, vis));
        }
        vis.remove(&vals[u]);
        vals[u] + best
    }
    let mut ans = i32::MIN;
    for u in 0..vals.len() {
        let mut vis = HashSet::new();
        ans = ans.max(dfs2(u, &vals, &adj, &mut vis));
    }
    ans
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 2,
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
    println!("{}", max_sum(root));
}

#[cfg(test)]
mod tests {
    use super::{max_sum, TreeNode};

    #[test]
    fn example1() {
        let root = Some(Box::new(TreeNode {
            val: 2,
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
        assert_eq!(max_sum(root), 3);
    }

    #[test]
    fn example2() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: -2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode {
                    val: 3,
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
        assert_eq!(max_sum(root), 9);
    }

    #[test]
    fn example3() {
        let root = Some(Box::new(TreeNode {
            val: 4,
            left: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                })),
            })),
        }));
        assert_eq!(max_sum(root), 19);
    }
}
