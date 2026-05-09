/// LeetCode #559 - Maximum Depth of N-ary Tree
#[derive(Debug)]
pub struct NaryNode {
    pub val: i32,
    pub children: Vec<Option<Box<NaryNode>>>,
}

fn max_depth(root: Option<Box<NaryNode>>) -> i32 {
    fn depth(node: &Option<Box<NaryNode>>) -> i32 {
        let Some(n) = node else { return 0 };
        let mut d = 0;
        for c in &n.children {
            d = d.max(depth(c));
        }
        d + 1
    }
    depth(&root)
}

fn main() {
    println!("{}", max_depth(None));
}

#[cfg(test)]
mod tests {
    use super::{max_depth, NaryNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(NaryNode {
            val: 1,
            children: vec![
                Some(Box::new(NaryNode {
                    val: 2,
                    children: vec![
                        Some(Box::new(NaryNode {
                            val: 5,
                            children: vec![],
                        })),
                        Some(Box::new(NaryNode {
                            val: 6,
                            children: vec![],
                        })),
                    ],
                })),
                Some(Box::new(NaryNode {
                    val: 3,
                    children: vec![],
                })),
                Some(Box::new(NaryNode {
                    val: 4,
                    children: vec![],
                })),
            ],
        }));
        assert_eq!(max_depth(root), 3);
    }
}
