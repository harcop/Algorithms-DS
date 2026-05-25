/// LeetCode #1382 - Balance A Binary Search Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn balance_bst(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let mut vals = vec![];
    fn inorder(node: Option<Box<TreeNode>>, vals: &mut Vec<i32>) {
        if let Some(n) = node {
            inorder(n.left, vals);
            vals.push(n.val);
            inorder(n.right, vals);
        }
    }
    inorder(root, &mut vals);
    fn build(vals: &[i32]) -> Option<Box<TreeNode>> {
        if vals.is_empty() {
            return None;
        }
        let mid = vals.len() / 2;
        Some(Box::new(TreeNode {
            val: vals[mid],
            left: build(&vals[..mid]),
            right: build(&vals[mid + 1..]),
        }))
    }
    build(&vals)
}

fn main() {
    let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
    r.right = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
    println!("{:?}", balance_bst(Some(r)).map(|n| n.val));
}

#[cfg(test)]
mod tests {
    use super::{balance_bst, TreeNode};

    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
        r.right = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
        r.right.as_mut().unwrap().right = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        r.right.as_mut().unwrap().right.as_mut().unwrap().right =
            Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        let b = balance_bst(Some(r)).unwrap();
        assert_eq!(b.val, 3);
    }
}
