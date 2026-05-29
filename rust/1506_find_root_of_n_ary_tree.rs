/// LeetCode #1506 - Find Root Of N Ary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node { pub val: i32, pub children: Vec<Box<Node>> }
fn find_root(tree: Vec<Option<Box<Node>>>) -> Option<Box<Node>> {
    let mut xor = 0i32;
    for node in &tree {
        if let Some(n) = node {
            xor ^= n.val;
            for c in &n.children { xor ^= c.val; }
        }
    }
    tree.into_iter().flatten().find(|n| n.val == xor)
}
fn main() {
    let n = Some(Box::new(Node { val: 1, children: vec![Box::new(Node { val: 3, children: vec![] })] }));
    println!("{}", find_root(vec![n, None]).unwrap().val);
}
#[cfg(test)]
mod tests {
    use super::{find_root, Node};
    #[test]
    fn example_one() {
        let root = Some(Box::new(Node { val: 1, children: vec![
            Box::new(Node { val: 3, children: vec![Box::new(Node { val: 5, children: vec![] })] }),
            Box::new(Node { val: 2, children: vec![] }),
        ]}));
        let c1 = Some(Box::new(Node { val: 3, children: vec![Box::new(Node { val: 5, children: vec![] })] }));
        let c2 = Some(Box::new(Node { val: 2, children: vec![] }));
        let c5 = Some(Box::new(Node { val: 5, children: vec![] }));
        assert_eq!(find_root(vec![root, c1, c2, None, None, c5]).unwrap().val, 1);
    }
}
