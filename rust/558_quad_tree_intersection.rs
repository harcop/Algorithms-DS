/// LeetCode #558 - Logical OR of Two Binary Grids Represented as Quad-Trees
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: bool,
    pub is_leaf: bool,
    pub top_left: Option<Box<Node>>,
    pub top_right: Option<Box<Node>>,
    pub bottom_left: Option<Box<Node>>,
    pub bottom_right: Option<Box<Node>>,
}

impl Node {
    fn leaf(val: bool) -> Self {
        Node {
            val,
            is_leaf: true,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }
}

fn intersect(t1: Option<Box<Node>>, t2: Option<Box<Node>>) -> Option<Box<Node>> {
    match (t1, t2) {
        (None, t) | (t, None) => t,
        (Some(a), Some(b)) => Some(Box::new(intersect_nodes(*a, *b))),
    }
}

fn intersect_nodes(t1: Node, t2: Node) -> Node {
    if t1.is_leaf {
        return if t1.val { t1 } else { t2 };
    }
    if t2.is_leaf {
        return if t2.val { t2 } else { t1 };
    }
    let tl = intersect_nodes(*t1.top_left.unwrap(), *t2.top_left.unwrap());
    let tr = intersect_nodes(*t1.top_right.unwrap(), *t2.top_right.unwrap());
    let bl = intersect_nodes(*t1.bottom_left.unwrap(), *t2.bottom_left.unwrap());
    let br = intersect_nodes(*t1.bottom_right.unwrap(), *t2.bottom_right.unwrap());
    if tl.is_leaf
        && tr.is_leaf
        && bl.is_leaf
        && br.is_leaf
        && tl.val == tr.val
        && tr.val == bl.val
        && bl.val == br.val
    {
        Node::leaf(tl.val)
    } else {
        Node {
            val: false,
            is_leaf: false,
            top_left: Some(Box::new(tl)),
            top_right: Some(Box::new(tr)),
            bottom_left: Some(Box::new(bl)),
            bottom_right: Some(Box::new(br)),
        }
    }
}

fn main() {
    let a = Some(Box::new(Node::leaf(true)));
    let b = Some(Box::new(Node::leaf(false)));
    println!("{:?}", intersect(a, b));
}

#[cfg(test)]
mod tests {
    use super::{intersect, Node};

    #[test]
    fn example_leaves() {
        let a = Some(Box::new(Node::leaf(true)));
        let b = Some(Box::new(Node::leaf(false)));
        assert_eq!(intersect(a, b), Some(Box::new(Node::leaf(true))));
    }

    #[test]
    fn example_merge() {
        let t1 = Node {
            val: false,
            is_leaf: false,
            top_left: Some(Box::new(Node::leaf(true))),
            top_right: Some(Box::new(Node::leaf(true))),
            bottom_left: Some(Box::new(Node::leaf(false))),
            bottom_right: Some(Box::new(Node::leaf(false))),
        };
        let t2 = Node {
            val: false,
            is_leaf: false,
            top_left: Some(Box::new(Node::leaf(true))),
            top_right: Some(Box::new(Node::leaf(true))),
            bottom_left: Some(Box::new(Node::leaf(true))),
            bottom_right: Some(Box::new(Node::leaf(false))),
        };
        let got = intersect(Some(Box::new(t1)), Some(Box::new(t2))).unwrap();
        assert!(!got.is_leaf);
        assert_eq!(got.top_left.as_ref().unwrap().val, true);
        assert_eq!(got.top_right.as_ref().unwrap().val, true);
        assert_eq!(got.bottom_left.as_ref().unwrap().val, true);
        assert_eq!(got.bottom_right.as_ref().unwrap().val, false);
    }
}
