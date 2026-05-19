/// LeetCode #1110 - Delete Nodes And Return Forest
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

use std::collections::HashSet;

fn del_nodes(
    root: Option<Box<TreeNode>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Box<TreeNode>>> {
    let del: HashSet<i32> = to_delete.into_iter().collect();
    let mut forest = Vec::new();
    fn dfs(
        node: Option<Box<TreeNode>>,
        del: &HashSet<i32>,
        forest: &mut Vec<Option<Box<TreeNode>>>,
        is_root: bool,
    ) -> Option<Box<TreeNode>> {
        let Some(mut node) = node else {
            return None;
        };
        let deleted = del.contains(&node.val);
        node.left = dfs(node.left, del, forest, deleted);
        node.right = dfs(node.right, del, forest, deleted);
        if deleted {
            return None;
        }
        if is_root {
            forest.push(Some(node));
            None
        } else {
            Some(node)
        }
    }
    dfs(root, &del, &mut forest, true);
    forest
}

fn main() {
    println!("{}", del_nodes(None, vec![3]).len());
}

#[cfg(test)]
mod tests {
    use super::{del_nodes, TreeNode};

    fn n(v: i32, l: Option<Box<TreeNode>>, r: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode { val: v, left: l, right: r }))
    }

    #[test]
    fn example_one() {
        let root = n(1, n(2, n(4, None, None), None), n(3, None, None));
        assert_eq!(del_nodes(root, vec![3]).len(), 1);
    }

    #[test]
    fn example_two() {
        let root = n(
            1,
            n(2, n(4, None, None), n(5, None, None)),
            n(3, n(6, n(7, None, None), None), None),
        );
        assert_eq!(del_nodes(root, vec![3, 5]).len(), 2);
    }
}
