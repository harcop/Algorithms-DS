/// LeetCode #257 - Binary Tree Paths
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn binary_tree_paths(root: Option<Box<TreeNode>>) -> Vec<String> {
    let mut out = vec![];
    fn dfs(node: &Option<Box<TreeNode>>, cur: &mut Vec<String>, out: &mut Vec<String>) {
        let Some(n) = node else { return };
        cur.push(n.val.to_string());
        if n.left.is_none() && n.right.is_none() {
            out.push(cur.join("->"));
        } else {
            dfs(&n.left, cur, out);
            dfs(&n.right, cur, out);
        }
        cur.pop();
    }
    let mut cur = vec![];
    dfs(&root, &mut cur, &mut out);
    out
}

fn main() {
    println!("{:?}", binary_tree_paths(None));
}

#[cfg(test)]
mod tests {
    use super::{binary_tree_paths, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
        });
        let mut v = binary_tree_paths(Some(root));
        v.sort();
        assert_eq!(v, vec!["1->2->5".to_string(), "1->3".to_string()]);
    }
}
