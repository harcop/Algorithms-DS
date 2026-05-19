/// LeetCode #1123 - Lowest Common Ancestor of Deepest Leaves
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn lca_deepest_leaves(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    fn dfs(node: Option<Box<TreeNode>>) -> (i32, Option<Box<TreeNode>>) {
        let Some(n) = node else {
            return (0, None);
        };
        let (dl, ll) = dfs(n.left);
        let (dr, lr) = dfs(n.right);
        let d = 1 + dl.max(dr);
        if dl == dr {
            (d, Some(n))
        } else if dl > dr {
            (d, ll)
        } else {
            (d, lr)
        }
    }
    dfs(root).1
}

fn main() {
    println!("{}", lca_deepest_leaves(None).is_none());
}

#[cfg(test)]
mod tests {
    use super::{lca_deepest_leaves, TreeNode};

    fn n(v: i32, l: Option<Box<TreeNode>>, r: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode { val: v, left: l, right: r }))
    }

    #[test]
    fn example_one() {
        let root = n(3, n(5, n(6, None, None), None), n(1, None, n(4, None, None)));
        assert_eq!(lca_deepest_leaves(root).unwrap().val, 5);
    }
}
