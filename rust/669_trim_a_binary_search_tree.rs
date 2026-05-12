/// LeetCode #669 - Trim a Binary Search Tree
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn trim_bst(root: Option<Box<TreeNode>>, low: i32, high: i32) -> Option<Box<TreeNode>> {
    let mut n = root?;
    if n.val < low {
        return trim_bst(n.right.take(), low, high);
    }
    if n.val > high {
        return trim_bst(n.left.take(), low, high);
    }
    n.left = trim_bst(n.left.take(), low, high);
    n.right = trim_bst(n.right.take(), low, high);
    Some(n)
}

fn main() {
    println!("{}", trim_bst(None, 1, 2).is_none());
}

#[cfg(test)]
mod tests {
    use super::{trim_bst, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode { val: 0, left: None, right: None })),
            right: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
        }));
        let r = trim_bst(root, 1, 2).unwrap();
        assert_eq!(r.val, 1);
        assert!(r.left.is_none());
        assert_eq!(r.right.as_ref().unwrap().val, 2);
    }
}
