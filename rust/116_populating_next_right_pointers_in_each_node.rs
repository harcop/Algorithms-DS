use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// LeetCode #116 - Populating Next Right Pointers in Each Node
#[derive(Debug, Clone)]
pub struct Node {
    pub val: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub next: RefCell<Option<Rc<RefCell<Node>>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
            next: RefCell::new(None),
        }
    }
}

fn connect(root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    let mut q = VecDeque::new();
    if let Some(r) = root.clone() {
        q.push_back(r);
    }
    while !q.is_empty() {
        let sz = q.len();
        let mut prev: Option<Rc<RefCell<Node>>> = None;
        for _ in 0..sz {
            let n = q.pop_front().unwrap();
            if let Some(p) = prev {
                *p.borrow().next.borrow_mut() = Some(n.clone());
            }
            prev = Some(n.clone());
            let b = n.borrow();
            if let Some(l) = b.left.clone() {
                q.push_back(l);
            }
            if let Some(r) = b.right.clone() {
                q.push_back(r);
            }
        }
    }
    root
}

fn main() {
    println!("{}", connect(None).is_none());
}

#[cfg(test)]
mod tests {
    use super::{connect, Node};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example_one() {
        let a = Rc::new(RefCell::new(Node::new(1)));
        let b = Rc::new(RefCell::new(Node::new(2)));
        let c = Rc::new(RefCell::new(Node::new(3)));
        let d = Rc::new(RefCell::new(Node::new(4)));
        let e = Rc::new(RefCell::new(Node::new(5)));
        let f = Rc::new(RefCell::new(Node::new(6)));
        let g = Rc::new(RefCell::new(Node::new(7)));
        a.borrow_mut().left = Some(b.clone());
        a.borrow_mut().right = Some(c.clone());
        b.borrow_mut().left = Some(d.clone());
        b.borrow_mut().right = Some(e.clone());
        c.borrow_mut().left = Some(f.clone());
        c.borrow_mut().right = Some(g.clone());

        connect(Some(a.clone()));

        assert!(b.borrow().next.borrow().as_ref().unwrap().borrow().val == 3);
        assert!(d.borrow().next.borrow().as_ref().unwrap().borrow().val == 5);
    }
}
