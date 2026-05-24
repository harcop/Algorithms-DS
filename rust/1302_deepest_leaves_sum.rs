/// LeetCode #1302 - Deepest Leaves Sum
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn deepest_leaves_sum(root: Option<Box<TreeNode>>) -> i32 {
    let mut q = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    let mut sum = 0;
    while !q.is_empty() {
        let sz = q.len();
        sum = 0;
        for _ in 0..sz {
            let node = q.pop_front().unwrap();
            sum += node.val;
            if let Some(l) = node.left {
                q.push_back(l);
            }
            if let Some(r) = node.right {
                q.push_back(r);
            }
        }
    }
    sum
}

fn main() {
    let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
    r.left = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
    r.right = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
    println!("{}", deepest_leaves_sum(Some(r)));
}

#[cfg(test)]
mod tests {
    use super::{deepest_leaves_sum, TreeNode};

    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
        r.left = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
        r.right = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        r.left.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        r.left.as_mut().unwrap().right = Some(Box::new(TreeNode { val: 5, left: None, right: None }));
        r.right.as_mut().unwrap().right = Some(Box::new(TreeNode { val: 6, left: None, right: None }));
        r.left.as_mut().unwrap().left.as_mut().unwrap().left =
            Some(Box::new(TreeNode { val: 7, left: None, right: None }));
        assert_eq!(deepest_leaves_sum(Some(r)), 7);
    }
}
