/// LeetCode #1367 - Linked List In Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn dfs_list(head: &Option<Box<ListNode>>, node: &Option<Box<TreeNode>>) -> bool {
    match (head, node) {
        (Some(h), Some(n)) => {
            if h.val != n.val {
                return false;
            }
            dfs_list(&h.next, &n.left) || dfs_list(&h.next, &n.right)
        }
        (None, _) => true,
        _ => false,
    }
}

fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Box<TreeNode>>) -> bool {
    fn walk(head: &Option<Box<ListNode>>, root: &Option<Box<TreeNode>>) -> bool {
        if dfs_list(head, root) {
            return true;
        }
        if let Some(n) = root {
            return walk(head, &n.left) || walk(head, &n.right);
        }
        false
    }
    walk(&head, &root)
}

fn main() {
    let list = Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 8, next: None })),
        })),
    }));
    let mut root = Box::new(TreeNode { val: 1, left: None, right: None });
    root.left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
    println!("{}", is_sub_path(list, Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{is_sub_path, ListNode, TreeNode};

    fn tree424() -> Option<Box<TreeNode>> {
        let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
        r.left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        r.left.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        r.left.as_mut().unwrap().right = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
        r.left.as_mut().unwrap().right.as_mut().unwrap().left =
            Some(Box::new(TreeNode { val: 1, left: None, right: None }));
        Some(r)
    }

    #[test]
    fn example_one() {
        let head = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));
        assert!(is_sub_path(head, tree424()));
    }
}
