use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// LeetCode #133 - Clone Graph
#[derive(Debug, Clone)]
pub struct Node {
    pub val: i32,
    pub neighbors: RefCell<Vec<Rc<RefCell<Node>>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: RefCell::new(vec![]),
        }
    }
}

fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    let mut map: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();

    fn dfs(
        node: &Rc<RefCell<Node>>,
        map: &mut HashMap<i32, Rc<RefCell<Node>>>,
    ) -> Rc<RefCell<Node>> {
        let v = node.borrow().val;
        if let Some(c) = map.get(&v) {
            return c.clone();
        }
        let copy = Rc::new(RefCell::new(Node::new(v)));
        map.insert(v, copy.clone());
        let neigh = node.borrow().neighbors.borrow().clone();
        for n in neigh {
            copy.borrow_mut()
                .neighbors
                .borrow_mut()
                .push(dfs(&n, map));
        }
        copy
    }

    node.map(|n| dfs(&n, &mut map))
}

fn main() {
    println!("{}", clone_graph(None).is_none());
}

#[cfg(test)]
mod tests {
    use super::{clone_graph, Node};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example_one() {
        let a = Rc::new(RefCell::new(Node::new(1)));
        let b = Rc::new(RefCell::new(Node::new(2)));
        let c = Rc::new(RefCell::new(Node::new(3)));
        let d = Rc::new(RefCell::new(Node::new(4)));
        a.borrow_mut().neighbors.borrow_mut().extend([b.clone(), d.clone()]);
        b.borrow_mut().neighbors.borrow_mut().extend([a.clone(), c.clone()]);
        c.borrow_mut().neighbors.borrow_mut().extend([b.clone(), d.clone()]);
        d.borrow_mut().neighbors.borrow_mut().extend([a.clone(), c.clone()]);

        let cloned = clone_graph(Some(a.clone())).unwrap();
        assert_eq!(cloned.borrow().val, 1);
        assert_eq!(cloned.borrow().neighbors.borrow().len(), 2);
    }
}
