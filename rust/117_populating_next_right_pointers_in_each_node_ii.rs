use std::cell::RefCell;
use std::rc::Rc;

/// LeetCode #117 - Populating Next Right Pointers in Each Node II
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
    let mut current_leftmost = root.clone();
    while let Some(level_start) = current_leftmost.clone() {
        let mut next_head: Option<Rc<RefCell<Node>>> = None;
        let mut next_tail: Option<Rc<RefCell<Node>>> = None;
        let mut cur = Some(level_start);

        while let Some(c) = cur {
            let (left, right, nxt) = {
                let b = c.borrow();
                let left = b.left.clone();
                let right = b.right.clone();
                let nxt = b.next.borrow().clone();
                (left, right, nxt)
            };
            for child in IntoIterator::into_iter([left, right]).flatten() {
                if next_head.is_none() {
                    next_head = Some(child.clone());
                    next_tail = Some(child.clone());
                } else if let Some(t) = &next_tail {
                    *t.borrow().next.borrow_mut() = Some(child.clone());
                    next_tail = Some(child.clone());
                }
            }
            cur = nxt;
        }
        current_leftmost = next_head;
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
        let root = Rc::new(RefCell::new(Node::new(1)));
        let n2 = Rc::new(RefCell::new(Node::new(2)));
        let n3 = Rc::new(RefCell::new(Node::new(3)));
        let n4 = Rc::new(RefCell::new(Node::new(4)));
        let n5 = Rc::new(RefCell::new(Node::new(5)));
        let n7 = Rc::new(RefCell::new(Node::new(7)));
        root.borrow_mut().left = Some(n2.clone());
        root.borrow_mut().right = Some(n3.clone());
        n2.borrow_mut().left = Some(n4.clone());
        n2.borrow_mut().right = Some(n5.clone());
        n3.borrow_mut().right = Some(n7.clone());

        connect(Some(root.clone()));

        assert_eq!(
            n4.borrow().next.borrow().as_ref().unwrap().borrow().val,
            5
        );
        assert_eq!(
            n5.borrow().next.borrow().as_ref().unwrap().borrow().val,
            7
        );
    }
}
