/// LeetCode #109 - Convert Sorted List to Binary Search Tree
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

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

fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Box<TreeNode>> {
    let mut vals = Vec::new();
    while let Some(node) = head {
        vals.push(node.val);
        head = node.next;
    }
    fn build(vals: &[i32]) -> Option<Box<TreeNode>> {
        if vals.is_empty() {
            return None;
        }
        let mid = vals.len() / 2;
        let mut root = Box::new(TreeNode::new(vals[mid]));
        root.left = build(&vals[..mid]);
        root.right = build(&vals[mid + 1..]);
        Some(root)
    }
    build(&vals)
}

fn vec_to_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut h = None;
    for &v in values.iter().rev() {
        let mut n = Box::new(ListNode::new(v));
        n.next = h;
        h = Some(n);
    }
    h
}

fn main() {
    println!("{:?}", sorted_list_to_bst(vec_to_list(&[-10, -3, 0, 5, 9])));
}

#[cfg(test)]
mod tests {
    use super::{sorted_list_to_bst, vec_to_list};

    #[test]
    fn example_one() {
        let root = sorted_list_to_bst(vec_to_list(&[-10, -3, 0, 5, 9]));
        assert!(root.is_some());
        assert_eq!(root.unwrap().val, 0);
    }
}
