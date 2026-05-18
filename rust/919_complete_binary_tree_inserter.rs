/// LeetCode #919 - Complete Binary Tree Inserter
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone)]
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

struct CbtInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    q: VecDeque<Rc<RefCell<TreeNode>>>,
}

impl CbtInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut q = VecDeque::new();
        if let Some(r) = root.clone() {
            q.push_back(r);
            loop {
                let front = q.front().unwrap().clone();
                let nb = front.borrow();
                if nb.left.is_some() && nb.right.is_some() {
                    let left = nb.left.as_ref().unwrap().clone();
                    let right = nb.right.as_ref().unwrap().clone();
                    drop(nb);
                    q.pop_front();
                    q.push_back(left);
                    q.push_back(right);
                } else {
                    break;
                }
            }
        }
        Self { root, q }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let parent = self.q.front().unwrap().clone();
        let child = Rc::new(RefCell::new(TreeNode::new(val)));
        let mut p = parent.borrow_mut();
        let v = p.val;
        if p.left.is_none() {
            p.left = Some(child.clone());
        } else {
            p.right = Some(child.clone());
            drop(p);
            self.q.pop_front();
        }
        self.q.push_back(child);
        v
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

fn collect_values(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut out = Vec::new();
    if root.is_none() {
        return out;
    }
    let mut dq = VecDeque::new();
    dq.push_back(root.as_ref().unwrap().clone());
    while let Some(n) = dq.pop_front() {
        let b = n.borrow();
        out.push(b.val);
        if let Some(ref l) = b.left {
            dq.push_back(l.clone());
        }
        if let Some(ref r) = b.right {
            dq.push_back(r.clone());
        }
    }
    out
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut ins = CbtInserter::new(root);
    println!("{}", ins.insert(2));
    println!("{:?}", collect_values(&ins.get_root()));
}

#[cfg(test)]
mod tests {
    use super::{CbtInserter, Rc, RefCell, TreeNode, collect_values};

    #[test]
    fn example() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut ins = CbtInserter::new(root);
        assert_eq!(ins.insert(2), 1);
        assert_eq!(collect_values(&ins.get_root()), vec![1, 2]);
    }
}
