/// LeetCode #687 - Longest Univalue Path
#[derive(Debug)]
pub struct TreeNode { pub val: i32, pub left: Option<Box<TreeNode>>, pub right: Option<Box<TreeNode>> }

fn longest_univalue_path(root: Option<Box<TreeNode>>) -> i32 {
    let mut best = 0i32;
    fn dfs(node: &Option<Box<TreeNode>>, best: &mut i32) -> i32 {
        let Some(n) = node else { return 0 };
        let l = dfs(&n.left, best);
        let r = dfs(&n.right, best);
        let mut left_arrow = 0; let mut right_arrow = 0;
        if let Some(ln) = &n.left { if ln.val == n.val { left_arrow = l + 1; } }
        if let Some(rn) = &n.right { if rn.val == n.val { right_arrow = r + 1; } }
        *best = (*best).max(left_arrow + right_arrow);
        left_arrow.max(right_arrow)
    }
    dfs(&root, &mut best);
    best
}

fn main() {
    println!("{}", longest_univalue_path(None));
}

#[cfg(test)]
mod tests {
    use super::{longest_univalue_path, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode {
                val: 4,
                left: Some(Box::new(TreeNode { val: 1, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 1, left: None, right: None })),
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Box::new(TreeNode { val: 5, left: None, right: None })),
            })),
        }));
        assert_eq!(longest_univalue_path(root), 2);
    }
}
