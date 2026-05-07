/// LeetCode #431 - Encode N-ary Tree to Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NAryNode {
    pub val: i32,
    pub children: Vec<NAryNode>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinNode {
    pub val: i32,
    pub left: Option<Box<BinNode>>,
    pub right: Option<Box<BinNode>>,
}

impl NAryNode {
    pub fn leaf(v: i32) -> Self {
        NAryNode {
            val: v,
            children: vec![],
        }
    }
}

fn encode(mut root: NAryNode) -> Option<Box<BinNode>> {
    fn sibling_chain(children: &[NAryNode]) -> Option<Box<BinNode>> {
        if children.is_empty() {
            None
        } else {
            Some(Box::new(BinNode {
                val: children[0].val,
                left: encode_children(&children[0]),
                right: sibling_chain(&children[1..]),
            }))
        }
    }

    fn encode_children(node: &NAryNode) -> Option<Box<BinNode>> {
        sibling_chain(&node.children)
    }

    let ch = std::mem::take(&mut root.children);
    Some(Box::new(BinNode {
        val: root.val,
        left: sibling_chain(&ch),
        right: None,
    }))
}

fn decode(root: BinNode) -> NAryNode {
    fn walk(mut cur: Option<&BinNode>) -> Vec<NAryNode> {
        let mut out = vec![];
        while let Some(n) = cur {
            out.push(NAryNode {
                val: n.val,
                children: walk(n.left.as_deref()),
            });
            cur = n.right.as_deref();
        }
        out
    }

    NAryNode {
        val: root.val,
        children: walk(root.left.as_deref()),
    }
}

fn main() {
    let root = NAryNode {
        val: 1,
        children: vec![
            NAryNode {
                val: 3,
                children: vec![NAryNode::leaf(5), NAryNode::leaf(6)],
            },
            NAryNode::leaf(2),
            NAryNode::leaf(4),
        ],
    };
    println!(
        "{}",
        encode(root.clone()).map(|x| x.val).unwrap_or(0)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let root = NAryNode {
            val: 1,
            children: vec![
                NAryNode {
                    val: 3,
                    children: vec![NAryNode::leaf(5), NAryNode::leaf(6)],
                },
                NAryNode::leaf(2),
                NAryNode::leaf(4),
            ],
        };

        let b = encode(root.clone()).unwrap();
        let again = decode(*b.clone());
        assert_eq!(again, root);
    }
}
