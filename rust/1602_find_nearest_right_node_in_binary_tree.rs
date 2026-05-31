/// LeetCode #1602 - Find Nearest Right Node In Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn find_nearest_right_node(root: Option<Box<TreeNode>>, u: i32) -> Option<Box<TreeNode>> {
    let mut level = vec![];
    let mut found = false;
    let mut q = vec![root];
    while !q.is_empty() {
        let mut next = vec![];
        level.clear();
        for node in q.drain(..) {
            if let Some(n) = node {
                level.push(n);
            }
        }
        for (i, n) in level.iter().enumerate() {
            if n.val == u {
                return if i + 1 < level.len() { Some(level[i + 1].clone()) } else { None };
            }
        }
        for n in level.drain(..) {
            if n.left.is_some() { next.push(n.left); }
            if n.right.is_some() { next.push(n.right); }
        }
        q = next;
    }
    None
}
fn main() { println!("{:?}", find_nearest_right_node(None, 1)); }
#[cfg(test)]
mod tests {
    use super::{find_nearest_right_node, TreeNode};
    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode { val: 1, left: Some(Box::new(TreeNode { val: 2, left: None, right: None })), right: Some(Box::new(TreeNode { val: 3, left: None, right: None })) }));
        assert_eq!(find_nearest_right_node(root, 2).map(|n| n.val), Some(3));
    }
}