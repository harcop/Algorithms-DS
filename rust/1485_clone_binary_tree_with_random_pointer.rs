/// LeetCode #1485 - Clone Binary Tree With Random Pointer
use std::collections::HashMap;
#[derive(Clone)]
pub struct Node {
    pub val: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub random: Option<Box<Node>>,
}
fn copy_random_binary_tree(root: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut map: HashMap<usize, Box<Node>> = HashMap::new();
    fn phase1(node: Option<&Node>, map: &mut HashMap<usize, Box<Node>>) {
        if let Some(n) = node {
            let k = n as *const Node as usize;
            if map.contains_key(&k) { return; }
            map.insert(k, Box::new(Node { val: n.val, left: None, right: None, random: None }));
            phase1(n.left.as_deref(), map);
            phase1(n.right.as_deref(), map);
            phase1(n.random.as_deref(), map);
        }
    }
    phase1(root.as_deref(), &mut map);
    let links: Vec<(usize, Option<usize>, Option<usize>, Option<usize>)> = map.keys().map(|&k| {
        let old = k as *const Node;
        unsafe {
            let n = &*old;
            (k,
                n.left.as_ref().map(|l| &**l as *const Node as usize),
                n.right.as_ref().map(|r| &**r as *const Node as usize),
                n.random.as_ref().map(|r| &**r as *const Node as usize))
        }
    }).collect();
    for (k, l, r, rnd) in links {
        if let Some(lk) = l {
            let left = map.remove(&lk);
            map.get_mut(&k).unwrap().left = left;
        }
        if let Some(rk) = r {
            let right = map.remove(&rk);
            map.get_mut(&k).unwrap().right = right;
        }
        if let Some(rk) = rnd {
            let random = map.remove(&rk);
            map.get_mut(&k).unwrap().random = random;
        }
    }
    root.map(|r| map.remove(&(r.as_ref() as *const Node as usize)).unwrap())
}
fn main() {
    let n = Box::new(Node { val: 1, left: None, right: None, random: None });
    println!("{}", copy_random_binary_tree(Some(n)).is_some());
}
#[cfg(test)]
mod tests {
    use super::{copy_random_binary_tree, Node};
    #[test]
    fn example_one() {
        let mut n = Box::new(Node { val: 1, left: None, right: None, random: None });
        n.left = Some(Box::new(Node { val: 2, left: None, right: None, random: None }));
        let c = copy_random_binary_tree(Some(n)).unwrap();
        assert_eq!(c.val, 1);
        assert_eq!(c.left.as_ref().unwrap().val, 2);
    }
}