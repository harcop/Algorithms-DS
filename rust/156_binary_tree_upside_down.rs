/// LeetCode #156 - Binary Tree Upside Down
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn upside_down_binary_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let mut curr = root;
    let mut prev: Option<Box<TreeNode>> = None;
    let mut tmp: Option<Box<TreeNode>> = None;

    while let Some(mut node) = curr {
        let next = node.left.take();
        let old_right = node.right.take();
        node.left = tmp.take();
        node.right = prev.take();
        prev = Some(node);
        tmp = old_right;
        curr = next;
    }
    prev
}

fn main() {
    println!("{}", upside_down_binary_tree(None).is_none());
}

#[cfg(test)]
mod tests {
    use super::{upside_down_binary_tree, TreeNode};

    fn collect_preorder(root: &Option<Box<TreeNode>>) -> Vec<i32> {
        let mut out = vec![];
        fn dfs(n: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
            if let Some(b) = n {
                out.push(b.val);
                dfs(&b.left, out);
                dfs(&b.right, out);
            }
        }
        dfs(root, &mut out);
        out
    }

    #[test]
    fn example_one() {
        let r = Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                })),
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
        let out = upside_down_binary_tree(Some(r));
        assert_eq!(collect_preorder(&out), vec![4, 5, 2, 3, 1]);
    }
}
