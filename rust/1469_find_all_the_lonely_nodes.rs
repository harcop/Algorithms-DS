/// LeetCode #1469 - Find All The Lonely Nodes
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}
fn get_lonely_nodes(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut res = Vec::new();
    fn dfs(node: Option<Box<TreeNode>>, res: &mut Vec<i32>) {
        if let Some(n) = node {
            if n.left.is_some() ^ n.right.is_some() {
                if n.left.is_some() { res.push(n.val); }
                if n.right.is_some() { res.push(n.val); }
            }
            dfs(n.left, res);
            dfs(n.right, res);
        }
    }
    dfs(root, &mut res);
    res
}
fn main() {
    let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
    r.left = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
    println!("{:?}", get_lonely_nodes(Some(r)));
}
#[cfg(test)]
mod tests {
    use super::{get_lonely_nodes, TreeNode};
    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
        r.left = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
        r.right = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        r.left.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        assert_eq!(get_lonely_nodes(Some(r)), vec![2]);
    }
}