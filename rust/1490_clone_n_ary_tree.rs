/// LeetCode #1490 - Clone N Ary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Box<Node>>,
}
fn clone_tree(root: Option<Box<Node>>) -> Option<Box<Node>> {
    root.map(|node| {
        Box::new(Node {
            val: node.val,
            children: node.children.into_iter().map(|c| clone_tree(Some(c)).unwrap()).collect(),
        })
    })
}
fn main() {
    let n = Box::new(Node { val: 1, children: vec![Box::new(Node { val: 3, children: vec![] })] });
    println!("{}", clone_tree(Some(n)).unwrap().val);
}
#[cfg(test)]
mod tests {
    use super::{clone_tree, Node};
    #[test]
    fn example_one() {
        let root = Some(Box::new(Node {
            val: 1,
            children: vec![
                Box::new(Node { val: 3, children: vec![Box::new(Node { val: 5, children: vec![] })] }),
                Box::new(Node { val: 2, children: vec![] }),
            ],
        }));
        let c = clone_tree(root).unwrap();
        assert_eq!(c.val, 1);
        assert_eq!(c.children.len(), 2);
        assert_eq!(c.children[0].val, 3);
    }
}