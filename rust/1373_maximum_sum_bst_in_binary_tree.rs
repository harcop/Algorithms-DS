/// LeetCode #1373 - Maximum Sum Bst In Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn max_sum_bst(root: Option<Box<TreeNode>>) -> i32 {
    let mut best = 0i32;
    fn dfs(node: &Option<Box<TreeNode>>, best: &mut i32) -> (bool, i32, i32, i32) {
        if node.is_none() {
            return (true, i32::MAX, i32::MIN, 0);
        }
        let n = node.as_ref().unwrap();
        let (lb, lmin, lmax, lsum) = dfs(&n.left, best);
        let (rb, rmin, rmax, rsum) = dfs(&n.right, best);
        if lb && rb && lmax < n.val && n.val < rmin {
            let sum = lsum + rsum + n.val;
            *best = (*best).max(sum);
            (true, lmin.min(n.val), rmax.max(n.val), sum)
        } else {
            (false, 0, 0, 0)
        }
    }
    dfs(&root, &mut best);
    best
}

fn main() {
    let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
    r.left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
    println!("{}", max_sum_bst(Some(r)));
}

#[cfg(test)]
mod tests {
    use super::{max_sum_bst, TreeNode};

    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
        r.left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        r.right = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        r.left.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        r.left.as_mut().unwrap().right = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
        assert_eq!(max_sum_bst(Some(r)), 3);
    }
}
