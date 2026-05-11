/// LeetCode #623 - Add One Row to Tree
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn add_one_row(root: Option<Box<TreeNode>>, val: i32, depth: i32) -> Option<Box<TreeNode>> {
    if depth == 1 {
        return Some(Box::new(TreeNode { val, left: root, right: None }));
    }
    fn dfs(node: &mut Option<Box<TreeNode>>, val: i32, depth: i32, cur: i32) {
        let Some(n) = node else { return };
        if cur == depth - 1 {
            let l = n.left.take();
            let r = n.right.take();
            n.left = Some(Box::new(TreeNode { val, left: l, right: None }));
            n.right = Some(Box::new(TreeNode { val, left: None, right: r }));
            return;
        }
        dfs(&mut n.left, val, depth, cur + 1);
        dfs(&mut n.right, val, depth, cur + 1);
    }
    let mut root = root;
    dfs(&mut root, val, depth, 1);
    root
}

fn main() {
    println!("{}", add_one_row(None, 1, 1).is_some());
}

#[cfg(test)]
mod tests {
    use super::{add_one_row, TreeNode};

    #[test]
    fn shallow_insert() {
        let root = Some(Box::new(TreeNode { val: 1, left: None, right: None }));
        let new_root = add_one_row(root, 5, 1).unwrap();
        assert_eq!(new_root.val, 5);
        assert_eq!(new_root.left.as_ref().unwrap().val, 1);
    }
}
