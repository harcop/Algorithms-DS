/// LeetCode #637 - Average of Levels in Binary Tree
use std::collections::VecDeque;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn average_of_levels(root: Option<Box<TreeNode>>) -> Vec<f64> {
    let mut out = vec![];
    let Some(r) = root else { return out };
    let mut q: VecDeque<Box<TreeNode>> = VecDeque::new();
    q.push_back(r);
    while !q.is_empty() {
        let cnt = q.len();
        let mut sum = 0i64;
        for _ in 0..cnt {
            let n = q.pop_front().unwrap();
            sum += n.val as i64;
            let mut n = n;
            if let Some(l) = n.left.take() { q.push_back(l); }
            if let Some(r) = n.right.take() { q.push_back(r); }
        }
        out.push(sum as f64 / cnt as f64);
    }
    out
}

fn main() {
    println!("{:?}", average_of_levels(None));
}

#[cfg(test)]
mod tests {
    use super::{average_of_levels, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode { val: 9, left: None, right: None })),
            right: Some(Box::new(TreeNode {
                val: 20,
                left: Some(Box::new(TreeNode { val: 15, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 7, left: None, right: None })),
            })),
        }));
        let v = average_of_levels(root);
        assert_eq!(v.len(), 3);
        assert!((v[0] - 3.0).abs() < 1e-9);
        assert!((v[1] - 14.5).abs() < 1e-9);
        assert!((v[2] - 11.0).abs() < 1e-9);
    }
}
