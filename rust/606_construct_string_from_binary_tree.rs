/// LeetCode #606 - Construct String from Binary Tree
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn tree2str(root: Option<Box<TreeNode>>) -> String {
    fn dfs(node: &Option<Box<TreeNode>>) -> String {
        let Some(n) = node else {
            return String::new();
        };
        let s = n.val.to_string();
        let ls = dfs(&n.left);
        let rs = dfs(&n.right);
        if n.left.is_none() && n.right.is_none() {
            return s;
        }
        if n.right.is_none() {
            return format!("{}({})", s, ls);
        }
        format!("{}({})({})", s, ls, rs)
    }
    dfs(&root)
}

fn main() {
    println!("{}", tree2str(None));
}

#[cfg(test)]
mod tests {
    use super::{tree2str, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            right: None,
        }));
        assert_eq!(tree2str(root), "1(2(4))");
    }
}
