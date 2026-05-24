/// LeetCode #1305 - All Elements in Two Binary Search Trees
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
    if let Some(n) = node {
        let n = n.borrow();
        inorder(&n.left, out);
        out.push(n.val);
        inorder(&n.right, out);
    }
}

fn get_all_elements(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<i32> {
    let mut a = vec![];
    let mut b = vec![];
    inorder(&root1, &mut a);
    inorder(&root2, &mut b);
    let mut i = 0;
    let mut j = 0;
    let mut ans = vec![];
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            ans.push(a[i]);
            i += 1;
        } else {
            ans.push(b[j]);
            j += 1;
        }
    }
    ans.extend_from_slice(&a[i..]);
    ans.extend_from_slice(&b[j..]);
    ans
}

fn main() {
    println!("{:?}", get_all_elements(None, None));
}

#[cfg(test)]
mod tests {
    use super::{get_all_elements, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn n(v: i32, l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode { val: v, left: l, right: r }))
    }

    #[test]
    fn example_one() {
        let r1 = n(2, Some(n(1, None, None)), Some(n(4, None, None)));
        let r2 = n(1, Some(n(0, None, None)), Some(n(3, None, None)));
        assert_eq!(get_all_elements(Some(r1), Some(r2)), vec![0, 1, 1, 2, 3, 4]);
    }
}
