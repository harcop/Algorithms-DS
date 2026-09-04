/// LeetCode #1120 - Maximum Average Subtree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn maximum_average_subtree(root: Option<Box<TreeNode>>) -> f64 {
    let mut best = f64::MIN;
    dfs(root.as_ref(), &mut best);
    best
}

fn dfs(node: Option<&Box<TreeNode>>, best: &mut f64) -> (i32, i32) {
    let Some(n) = node else {
        return (0, 0);
    };
    let (ls, lc) = dfs(n.left.as_ref(), best);
    let (rs, rc) = dfs(n.right.as_ref(), best);
    let sum = ls + rs + n.val;
    let cnt = lc + rc + 1;
    *best = best.max(sum as f64 / cnt as f64);
    (sum, cnt)
}

fn main() {
    let mut root = Box::new(TreeNode::new(5));
    root.left = Some(Box::new(TreeNode::new(6)));
    root.right = Some(Box::new(TreeNode::new(1)));
    println!("{}", maximum_average_subtree(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{maximum_average_subtree, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(5));
        root.left = Some(Box::new(TreeNode::new(6)));
        root.right = Some(Box::new(TreeNode::new(1)));
        let ans = maximum_average_subtree(Some(root));
        assert!((ans - 6.0).abs() < 1e-5);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(0));
        root.right = Some(Box::new(TreeNode::new(1)));
        let ans = maximum_average_subtree(Some(root));
        assert!((ans - 1.0).abs() < 1e-5);
    }
}
