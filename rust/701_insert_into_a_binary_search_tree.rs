/// LeetCode #701 - Insert into a Binary Search Tree
#[derive(Debug)]
pub struct TreeNode { pub val: i32, pub left: Option<Box<TreeNode>>, pub right: Option<Box<TreeNode>> }

fn insert_into_bst(root: Option<Box<TreeNode>>, val: i32) -> Option<Box<TreeNode>> {
    let Some(mut n) = root else {
        return Some(Box::new(TreeNode { val, left: None, right: None }));
    };
    if val < n.val { n.left = insert_into_bst(n.left.take(), val); }
    else { n.right = insert_into_bst(n.right.take(), val); }
    Some(n)
}

fn main() {
    println!("{}", insert_into_bst(None, 1).unwrap().val);
}

#[cfg(test)]
mod tests {
    use super::{insert_into_bst, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        let r = insert_into_bst(root, 5).unwrap();
        assert_eq!(r.right.as_ref().unwrap().val, 5);
    }
}
