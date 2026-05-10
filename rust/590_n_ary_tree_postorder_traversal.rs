/// LeetCode #590 - N-ary Tree Postorder Traversal
#[derive(Debug, Clone)]
pub struct NaryNode {
    pub val: i32,
    pub children: Vec<Option<Box<NaryNode>>>,
}

fn postorder(root: Option<Box<NaryNode>>) -> Vec<i32> {
    let mut out = vec![];
    fn dfs(node: &Option<Box<NaryNode>>, out: &mut Vec<i32>) {
        let Some(n) = node else { return };
        for c in &n.children {
            dfs(c, out);
        }
        out.push(n.val);
    }
    dfs(&root, &mut out);
    out
}

fn main() {
    println!("{:?}", postorder(None));
}

#[cfg(test)]
mod tests {
    use super::{postorder, NaryNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(NaryNode {
            val: 1,
            children: vec![
                Some(Box::new(NaryNode {
                    val: 3,
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
                    val: 2,
                    children: vec![],
                })),
                Some(Box::new(NaryNode {
                    val: 4,
                    children: vec![],
                })),
            ],
        }));
        assert_eq!(postorder(root), vec![5, 6, 3, 2, 4, 1]);
    }
}
