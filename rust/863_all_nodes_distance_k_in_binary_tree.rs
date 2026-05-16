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

fn distance_k(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: Rc<RefCell<TreeNode>>,
    k: i32,
) -> Vec<i32> {
    let mut parent: HashMap<usize, usize> = HashMap::new();
    let mut nodes: HashMap<usize, Rc<RefCell<TreeNode>>> = HashMap::new();

    fn build(
        node: Option<Rc<RefCell<TreeNode>>>,
        par: Option<usize>,
        parent: &mut HashMap<usize, usize>,
        nodes: &mut HashMap<usize, Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(n) = node {
            let id = Rc::as_ptr(&n) as usize;
            nodes.insert(id, n.clone());
            if let Some(p) = par {
                parent.insert(id, p);
            }
            let l = n.borrow().left.clone();
            let r = n.borrow().right.clone();
            build(l, Some(id), parent, nodes);
            build(r, Some(id), parent, nodes);
        }
    }
    build(root, None, &mut parent, &mut nodes);

    let start = Rc::as_ptr(&target) as usize;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((start, 0));
    seen.insert(start);
    let mut ans = Vec::new();
    while let Some((id, d)) = q.pop_front() {
        if d == k {
            ans.push(nodes[&id].borrow().val);
        }
        if d < k {
            let node = nodes[&id].clone();
            let mut next = Vec::new();
            if let Some(l) = node.borrow().left.clone() {
                next.push(Rc::as_ptr(&l) as usize);
            }
            if let Some(r) = node.borrow().right.clone() {
                next.push(Rc::as_ptr(&r) as usize);
            }
            if let Some(&p) = parent.get(&id) {
                next.push(p);
            }
            for nid in next {
                if seen.insert(nid) {
                    q.push_back((nid, d + 1));
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

    fn build_tree() -> (Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>) {
        let n = |v: i32| Rc::new(RefCell::new(TreeNode { val: v, left: None, right: None }));
        let n8 = n(8);
        let n0 = {
            let x = n(0);
            x.borrow_mut().left = Some(n8);
            x
        };
        let n6 = {
            let x = n(6);
            x.borrow_mut().left = Some(n0);
            x
        };
        let n7 = n(7);
        let n4 = n(4);
        let n2 = {
            let x = n(2);
            x.borrow_mut().left = Some(n7);
            x.borrow_mut().right = Some(n4);
            x
        };
        let n5 = {
            let x = n(5);
            x.borrow_mut().left = Some(n6);
            x.borrow_mut().right = Some(n2);
            x
        };
        let n1 = n(1);
        let root = {
            let x = n(3);
            x.borrow_mut().left = Some(n5.clone());
            x.borrow_mut().right = Some(n1);
            x
        };
        (root, n5)
    }

    #[test]
    fn example_one() {
        let (root, target) = build_tree();
        let mut got = distance_k(Some(root), target, 2);
        got.sort_unstable();
        assert_eq!(got, vec![0, 1, 4, 7]);
    }
}
