/// LeetCode #297 - Serialize and Deserialize Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn serialize(root: Option<Box<TreeNode>>) -> String {
    fn dfs(n: &Option<Box<TreeNode>>, out: &mut Vec<String>) {
        match n {
            None => out.push("#".into()),
            Some(b) => {
                out.push(b.val.to_string());
                dfs(&b.left, out);
                dfs(&b.right, out);
            }
        }
    }
    let mut out = vec![];
    dfs(&root, &mut out);
    out.join(",")
}

fn deserialize(data: String) -> Option<Box<TreeNode>> {
    let tokens: Vec<&str> = data.split(',').collect();
    let mut i = 0usize;
    fn build(tokens: &[&str], i: &mut usize) -> Option<Box<TreeNode>> {
        if *i >= tokens.len() || tokens[*i] == "#" {
            *i += 1;
            return None;
        }
        let val: i32 = tokens[*i].parse().unwrap();
        *i += 1;
        let left = build(tokens, i);
        let right = build(tokens, i);
        Some(Box::new(TreeNode {
            val,
            left,
            right,
        }))
    }
    build(&tokens, &mut i)
}

fn main() {
    println!("{}", serialize(None));
}

#[cfg(test)]
mod tests {
    use super::{deserialize, serialize, TreeNode};

    #[test]
    fn round_trip() {
        let root = Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
        });
        let s = serialize(Some(root));
        let back = deserialize(s.clone());
        assert_eq!(serialize(back), s);
    }
}
