/// LeetCode #1372 - Longest Zigzag Path In A Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn longest_zig_zag(root: Option<Box<TreeNode>>) -> i32 {
    let mut ans = 0i32;
    fn dfs(node: &Option<Box<TreeNode>>, ans: &mut i32) -> (i32, i32) {
        if let Some(n) = node {
            let (l0, l1) = dfs(&n.left, ans);
            let (r0, r1) = dfs(&n.right, ans);
            let left_zig = r1 + 1;
            let right_zig = l0 + 1;
            *ans = (*ans).max(left_zig).max(right_zig);
            (left_zig, right_zig)
        } else {
            (0, 0)
        }
    }
    dfs(&root, &mut ans);
    ans
}

fn main() {
    let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
    r.right = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
    println!("{}", longest_zig_zag(Some(r)));
}

#[cfg(test)]
mod tests {
    use super::{longest_zig_zag, TreeNode};

    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
        r.right = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
        r.right.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        r.right.as_mut().unwrap().right = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        r.right.as_mut().unwrap().left.as_mut().unwrap().right =
            Some(Box::new(TreeNode { val: 5, left: None, right: None }));
        assert_eq!(longest_zig_zag(Some(r)), 4);
    }
}
