/// LeetCode #863 - All Nodes Distance K in Binary Tree
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

fn distance_k(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: Rc<RefCell<TreeNode>>,
    k: i32,
) -> Vec<i32> {
    let mut parent = HashMap::new();
    fn build(
        node: Option<Rc<RefCell<TreeNode>>>,
        par: Option<Rc<RefCell<TreeNode>>>,
        parent: &mut HashMap<Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(n) = node {
            if let Some(p) = par {
                parent.insert(n.clone(), p);
            }
            let l = n.borrow().left.clone();
            let r = n.borrow().right.clone();
            build(l, Some(n.clone()), parent);
            build(r, Some(n.clone()), parent);
        }
    }
    build(root, None, &mut parent);
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((target.clone(), 0));
    seen.insert(target);
    let mut ans = Vec::new();
    while let Some((node, d)) = q.pop_front() {
        if d == k {
            ans.push(node.borrow().val);
        }
        if d < k {
            let neighbors = [
                node.borrow().left.clone(),
                node.borrow().right.clone(),
                parent.get(&node).cloned(),
            ];
            for nb in neighbors.into_iter().flatten() {
                if seen.insert(nb.clone()) {
                    q.push_back((nb, d + 1));
                }
            }
        }
    }
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::{distance_k, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example_one() {
        let mut two = Rc::new(RefCell::new(TreeNode::new(2)));
        two.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        two.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let mut five = Rc::new(RefCell::new(TreeNode::new(5)));
        five.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        five.borrow_mut().right = Some(two);
        let mut root = Rc::new(RefCell::new(TreeNode::new(3)));
        root.borrow_mut().left = Some(five.clone());
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut got = distance_k(Some(root), five, 2);
        got.sort_unstable();
        assert_eq!(got, vec![7, 4]);
    }
}
