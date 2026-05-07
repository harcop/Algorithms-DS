/// LeetCode #449 - Serialize and Deserialize BST
#[derive(Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }

    fn serialize(&self, root: Option<Box<TreeNode>>) -> String {
        fn dfs(n: &Option<Box<TreeNode>>, out: &mut Vec<String>) {
            match n {
                None => out.push("#".into()),
                Some(node) => {
                    out.push(node.val.to_string());
                    dfs(&node.left, out);
                    dfs(&node.right, out);
                }
            }
        }
        let mut out = vec![];
        dfs(&root, &mut out);
        out.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Box<TreeNode>> {
        let mut it = data.split(',').peekable();
        fn dfs(it: &mut std::iter::Peekable<std::str::Split<'_, char>>) -> Option<Box<TreeNode>> {
            let t = it.next()?;
            if t.is_empty() || t == "#" {
                return None;
            }
            let val: i32 = t.parse().ok()?;
            Some(Box::new(TreeNode {
                val,
                left: dfs(it),
                right: dfs(it),
            }))
        }
        dfs(&mut it)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Codec, TreeNode};

    #[test]
    fn round_trip() {
        let c = Codec::new();
        let root = Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
        }));
        let s = c.serialize(root.clone());
        assert_eq!(c.serialize(c.deserialize(s)), c.serialize(root));
    }
}
