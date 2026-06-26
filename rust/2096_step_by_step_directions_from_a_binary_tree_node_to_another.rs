/// LeetCode #2096 - Step-By-Step Directions From a Binary Tree Node to Another
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn get_directions(
    root: Option<Rc<RefCell<TreeNode>>>,
    start_value: i32,
    dest_value: i32,
) -> String {
    fn path(
        node: &Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        cur: &mut Vec<char>,
        out: &mut Vec<char>,
    ) -> bool {
        let Some(node) = node else {
            return false;
        };
        let node = node.borrow();
        if node.val == target {
            *out = cur.clone();
            return true;
        }

        cur.push('L');
        if path(&node.left, target, cur, out) {
            return true;
        }
        cur.pop();

        cur.push('R');
        if path(&node.right, target, cur, out) {
            return true;
        }
        cur.pop();

        false
    }

    let mut to_start = Vec::new();
    let mut to_dest = Vec::new();
    path(&root, start_value, &mut Vec::new(), &mut to_start);
    path(&root, dest_value, &mut Vec::new(), &mut to_dest);

    let mut i = 0usize;
    while i < to_start.len() && i < to_dest.len() && to_start[i] == to_dest[i] {
        i += 1;
    }

    let mut ans = String::new();
    ans.extend(std::iter::repeat('U').take(to_start.len() - i));
    ans.extend(to_dest[i..].iter());
    ans
}

fn main() {
    let root = node(
        5,
        Some(node(1, Some(node(3, None, None)), None)),
        Some(node(2, Some(node(6, None, None)), Some(node(4, None, None)))),
    );
    println!("{}", get_directions(Some(root), 3, 6));
}

fn node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode { val, left, right }))
}

#[cfg(test)]
mod tests {
    use super::{get_directions, node};

    #[test]
    fn example_one() {
        let root = node(
            5,
            Some(node(1, Some(node(3, None, None)), None)),
            Some(node(2, Some(node(6, None, None)), Some(node(4, None, None)))),
        );
        assert_eq!(get_directions(Some(root), 3, 6), "UURL");
    }

    #[test]
    fn example_two() {
        let root = node(2, Some(node(1, None, None)), None);
        assert_eq!(get_directions(Some(root), 2, 1), "L");
    }
}
