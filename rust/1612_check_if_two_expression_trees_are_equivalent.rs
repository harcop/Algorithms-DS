/// LeetCode #1612 - Check If Two Expression Trees Are Equivalent
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: String,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

fn eval(node: &Node) -> i64 {
    if node.left.is_none() { return node.val.parse().unwrap(); }
    let l = eval(node.left.as_ref().unwrap());
    let r = eval(node.right.as_ref().unwrap());
    match node.val.as_str() {
        "+" => l + r,
        "-" => l - r,
        "*" => l * r,
        "/" => l / r,
        _ => 0,
    }
}

fn check_equivalence(root1: Option<Box<Node>>, root2: Option<Box<Node>>) -> bool {
    match (root1, root2) {
        (None, None) => true,
        (Some(r1), Some(r2)) => eval(&r1) == eval(&r2),
        _ => false,
    }
}
fn main() { println!("{}", check_equivalence(None, None)); }
#[cfg(test)]
mod tests {
    use super::{check_equivalence, Node};
    fn leaf(v: &str) -> Option<Box<Node>> {
        Some(Box::new(Node { val: v.into(), left: None, right: None }))
    }
    #[test]
    fn example_one() {
        let a = Some(Box::new(Node { val: "+".into(), left: leaf("2"), right: leaf("3") }));
        let b = Some(Box::new(Node { val: "*".into(), left: leaf("1"), right: leaf("5") }));
        assert!(check_equivalence(a, b));
    }
}