/// LeetCode #1430 - Check If A String Is A Valid Sequence From Root To Leaves Path In A Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn is_valid_sequence(root: Option<Box<TreeNode>>, arr: Vec<i32>) -> bool {
    fn dfs(node: Option<Box<TreeNode>>, arr: &[i32], i: usize) -> bool {
        if i == arr.len() {
            return false;
        }
        let Some(n) = node else { return false };
        if n.val != arr[i] {
            return false;
        }
        if i == arr.len() - 1 {
            return n.left.is_none() && n.right.is_none();
        }
        dfs(n.left, arr, i + 1) || dfs(n.right, arr, i + 1)
    }
    dfs(root, &arr, 0)
}

fn main() {
    let leaf = |v| Some(Box::new(TreeNode { val: v, left: None, right: None }));
    let mut root = Box::new(TreeNode { val: 0, left: leaf(1), right: leaf(2) });
    root.left.as_mut().unwrap().left = leaf(3);
    root.left.as_mut().unwrap().right = leaf(4);
    println!("{}", is_valid_sequence(Some(root), vec![0, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::{is_valid_sequence, TreeNode};

    fn leaf(v: i32) -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode { val: v, left: None, right: None }))
    }

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode { val: 0, left: leaf(1), right: leaf(2) });
        root.left.as_mut().unwrap().left = leaf(3);
        root.left.as_mut().unwrap().right = leaf(4);
        assert!(is_valid_sequence(Some(root), vec![0, 1, 3]));
    }
}

