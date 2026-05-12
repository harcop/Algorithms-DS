/// LeetCode #700 - Search in a Binary Search Tree
#[derive(Debug)]
pub struct TreeNode { pub val: i32, pub left: Option<Box<TreeNode>>, pub right: Option<Box<TreeNode>> }

fn search_bst(root: Option<Box<TreeNode>>, val: i32) -> Option<Box<TreeNode>> {
    let n = root?;
    if n.val == val { return Some(n); }
    if val < n.val { search_bst(n.left, val) } else { search_bst(n.right, val) }
}

fn main() {
    println!("{}", search_bst(None, 1).is_none());
}

#[cfg(test)]
mod tests {
    use super::{search_bst, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 4,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode { val: 1, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 3, left: None, right: None })),
            })),
            right: Some(Box::new(TreeNode { val: 7, left: None, right: None })),
        }));
        let r = search_bst(root, 2).unwrap();
        assert_eq!(r.val, 2);
    }
}
