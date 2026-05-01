use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// LeetCode #138 - Copy List with Random Pointer
#[derive(Debug, Clone)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: RefCell<Option<Rc<RefCell<Node>>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            next: None,
            random: RefCell::new(None),
        }
    }
}

fn ptr_key(n: &Rc<RefCell<Node>>) -> usize {
    Rc::as_ptr(n) as usize
}

fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    let mut map: HashMap<usize, Rc<RefCell<Node>>> = HashMap::new();
    let mut cur = head.clone();
    while let Some(n) = cur {
        let k = ptr_key(&n);
        let c = Rc::new(RefCell::new(Node::new(n.borrow().val)));
        map.insert(k, c);
        cur = n.borrow().next.clone();
    }
    cur = head.clone();
    while let Some(n) = cur {
        let c = map[&ptr_key(&n)].clone();
        if let Some(next) = n.borrow().next.clone() {
            c.borrow_mut().next = Some(map[&ptr_key(&next)].clone());
        }
        if let Some(r) = n.borrow().random.borrow().clone() {
            *c.borrow().random.borrow_mut() = Some(map[&ptr_key(&r)].clone());
        }
        cur = n.borrow().next.clone();
    }
    head.map(|h| map[&ptr_key(&h)].clone())
}

fn main() {
    println!("{}", copy_random_list(None).is_none());
}

#[cfg(test)]
mod tests {
    use super::{copy_random_list, Node};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example_one() {
        let a = Rc::new(RefCell::new(Node::new(7)));
        let b = Rc::new(RefCell::new(Node::new(13)));
        let c = Rc::new(RefCell::new(Node::new(11)));
        let d = Rc::new(RefCell::new(Node::new(10)));
        let e = Rc::new(RefCell::new(Node::new(1)));
        a.borrow_mut().next = Some(b.clone());
        b.borrow_mut().next = Some(c.clone());
        c.borrow_mut().next = Some(d.clone());
        d.borrow_mut().next = Some(e.clone());
        *b.borrow().random.borrow_mut() = Some(a.clone());
        *c.borrow().random.borrow_mut() = Some(e.clone());
        *d.borrow().random.borrow_mut() = Some(c.clone());
        *e.borrow().random.borrow_mut() = Some(a.clone());

        let h = copy_random_list(Some(a.clone())).unwrap();
        assert_eq!(h.borrow().val, 7);
        assert_eq!(h.borrow().next.as_ref().unwrap().borrow().val, 13);
    }
}
