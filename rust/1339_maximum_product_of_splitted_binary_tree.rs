/// LeetCode #1339 - Maximum Product Of Splitted Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn tree_sum(node: &Option<Box<TreeNode>>) -> i64 {
    node.as_ref()
        .map(|n| n.val as i64 + tree_sum(&n.left) + tree_sum(&n.right))
        .unwrap_or(0)
}

fn max_product(root: Option<Box<TreeNode>>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let total = tree_sum(&root);
    let mut best = 0i64;
    fn dfs(node: &Option<Box<TreeNode>>, total: i64, best: &mut i64) -> i64 {
        let Some(n) = node else { return 0 };
        let ls = dfs(&n.left, total, best);
        let rs = dfs(&n.right, total, best);
        if n.left.is_some() {
            *best = (*best).max(ls * (total - ls));
        }
        if n.right.is_some() {
            *best = (*best).max(rs * (total - rs));
        }
        n.val as i64 + ls + rs
    }
    dfs(&root, total, &mut best);
    (best % MOD) as i32
}

fn main() {
    let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
    r.left = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
    r.right = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
    r.left.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
    r.left.as_mut().unwrap().right = Some(Box::new(TreeNode { val: 5, left: None, right: None }));
    r.right.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 6, left: None, right: None }));
    println!("{}", max_product(Some(r)));
}

#[cfg(test)]
mod tests {
    use super::{max_product, TreeNode};

    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
        r.left = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
        r.right = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        r.left.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        r.left.as_mut().unwrap().right = Some(Box::new(TreeNode { val: 5, left: None, right: None }));
        r.right.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 6, left: None, right: None }));
        assert_eq!(max_product(Some(r)), 110);
    }
}
