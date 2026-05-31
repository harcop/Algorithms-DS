/// LeetCode #1609 - Even Odd Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn is_even_odd_tree(root: Option<Box<TreeNode>>) -> bool {
    let mut q = vec![root];
    let mut level = 0usize;
    while !q.is_empty() {
        let mut nq = vec![];
        let mut vals = vec![];
        for node in q.drain(..) {
            if let Some(n) = node {
                vals.push(n.val);
                if n.left.is_some() { nq.push(n.left); }
                if n.right.is_some() { nq.push(n.right); }
            }
        }
        if level % 2 == 0 {
            for i in 1..vals.len() {
                if vals[i] <= vals[i - 1] || vals[i] % 2 == 0 { return false; }
            }
            if !vals.is_empty() && vals[0] % 2 == 0 { return false; }
        } else {
            for i in 1..vals.len() {
                if vals[i] >= vals[i - 1] || vals[i] % 2 == 1 { return false; }
            }
            if !vals.is_empty() && vals[0] % 2 == 1 { return false; }
        }
        level += 1;
        q = nq;
    }
    true
}
fn main() { println!("{}", is_even_odd_tree(None)); }
#[cfg(test)]
mod tests {
    use super::{is_even_odd_tree, TreeNode};
    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode { val: 1, left: Some(Box::new(TreeNode { val: 10, left: None, right: None })), right: Some(Box::new(TreeNode { val: 4, left: None, right: None })) }));
        assert!(is_even_odd_tree(root));
    }
}