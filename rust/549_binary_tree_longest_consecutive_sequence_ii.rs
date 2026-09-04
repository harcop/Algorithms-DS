/// LeetCode #549 - Binary Tree Longest Consecutive Sequence II
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

fn longest_consecutive(root: Option<Box<TreeNode>>) -> i32 {
    let mut ans = 0;
    fn dfs(node: &Option<Box<TreeNode>>, ans: &mut i32) -> (i32, i32) {
        let Some(n) = node else {
            return (0, 0);
        };
        let (li, ld) = dfs(&n.left, ans);
        let (ri, rd) = dfs(&n.right, ans);
        let mut inc = 1;
        let mut dec = 1;
        if let Some(l) = &n.left {
            if l.val == n.val + 1 {
                inc = inc.max(li + 1);
            }
            if l.val == n.val - 1 {
                dec = dec.max(ld + 1);
            }
        }
        if let Some(r) = &n.right {
            if r.val == n.val + 1 {
                inc = inc.max(ri + 1);
            }
            if r.val == n.val - 1 {
                dec = dec.max(rd + 1);
            }
        }
        *ans = (*ans).max(inc + dec - 1);
        (inc, dec)
    }
    dfs(&root, &mut ans);
    ans
}

fn main() {
    let mut root = Box::new(TreeNode::new(1));
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(3)));
    println!("{}", longest_consecutive(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{longest_consecutive, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(3)));
        assert_eq!(longest_consecutive(Some(root)), 2);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(2));
        root.left = Some(Box::new(TreeNode::new(1)));
        root.right = Some(Box::new(TreeNode::new(3)));
        assert_eq!(longest_consecutive(Some(root)), 3);
    }
}
