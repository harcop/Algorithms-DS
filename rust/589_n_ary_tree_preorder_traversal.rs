/// LeetCode #589 - N-ary Tree Preorder Traversal
#[derive(Debug, Clone)]
pub struct NaryNode {
    pub val: i32,
    pub children: Vec<Option<Box<NaryNode>>>,
}

fn preorder(root: Option<Box<NaryNode>>) -> Vec<i32> {
    let mut out = vec![];
    fn dfs(node: &Option<Box<NaryNode>>, out: &mut Vec<i32>) {
        let Some(n) = node else { return };
        out.push(n.val);
        for c in &n.children {
            dfs(c, out);
        }
    }
    dfs(&root, &mut out);
    out
}

fn main() {
    println!("{:?}", preorder(None));
}

#[cfg(test)]
mod tests {
    use super::{preorder, NaryNode};

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
        assert_eq!(preorder(root), vec![1, 3, 5, 6, 2, 4]);
    }
}
