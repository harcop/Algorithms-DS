/// LeetCode #987 - Vertical Order Traversal of a Binary Tree
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut nodes: Vec<(i32, i32, i32)> = Vec::new();
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, row: i32, col: i32, out: &mut Vec<(i32, i32, i32)>) {
        if node.is_none() { return; }
        let n = node.as_ref().unwrap().borrow();
        out.push((col, row, n.val));
        dfs(&n.left, row + 1, col - 1, out);
        dfs(&n.right, row + 1, col + 1, out);
    }
    dfs(&root, 0, 0, &mut nodes);
    nodes.sort_by(|a, b| (a.0, a.1, a.2).cmp(&(b.0, b.1, b.2)));
    let mut map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for (col, _row, val) in nodes {
        map.entry(col).or_default().push(val);
    }
    map.into_values().collect()
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    println!("{:?}", vertical_traversal(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = Rc::new(RefCell::new(TreeNode::new(3)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        root.borrow().right.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(15))));
        root.borrow().right.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(vertical_traversal(Some(root)), vec![vec![9], vec![3, 15], vec![20], vec![7]]);
    }
}
