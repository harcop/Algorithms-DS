/// LeetCode #1080 - Insufficient Nodes in Root to Leaf Paths
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn sufficient_subset(root: Option<Box<TreeNode>>, limit: i32) -> Option<Box<TreeNode>> {
    fn dfs(node: Option<Box<TreeNode>>, limit: i32) -> (Option<Box<TreeNode>>, i32, i32) {
        let Some(mut node) = node else {
            return (None, i32::MIN / 2, i32::MAX / 2);
        };
        if node.left.is_none() && node.right.is_none() {
            let v = node.val;
            return (Some(node), v, v);
        }
        let (l, lmax, lmin) = dfs(node.left.take(), limit - node.val);
        let (r, rmax, rmin) = dfs(node.right.take(), limit - node.val);
        let max_path = lmax.max(rmax);
        let min_path = lmin.min(rmin);
        if max_path < limit {
            return (None, max_path, min_path);
        }
        node.left = l;
        node.right = r;
        (Some(node), max_path, min_path)
    }
    dfs(root, limit).0
}

fn main() {
    let _ = sufficient_subset(None, 0);
}

#[cfg(test)]
mod tests {
    use super::{sufficient_subset, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode { val: -99, left: None, right: None })),
                right: None,
            })),
            right: None,
        }));
        assert!(sufficient_subset(root, 1).is_none());
    }
}
