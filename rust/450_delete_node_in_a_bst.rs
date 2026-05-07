/// LeetCode #450 - Delete Node in a BST
#[derive(Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn min_tree_val(root: &TreeNode) -> i32 {
    let mut cur = root;
    while let Some(ref l) = cur.left {
        cur = l;
    }
    cur.val
}

fn delete_node(mut root: Option<Box<TreeNode>>, key: i32) -> Option<Box<TreeNode>> {
    match root.take() {
        None => None,
        Some(mut n) => {
            if key < n.val {
                n.left = delete_node(n.left.take(), key);
                Some(n)
            } else if key > n.val {
                n.right = delete_node(n.right.take(), key);
                Some(n)
            } else {
                if n.left.is_none() {
                    return n.right;
                }
                if n.right.is_none() {
                    return n.left;
                }
                let succ_val = min_tree_val(n.right.as_ref().unwrap());
                n.val = succ_val;
                n.right = delete_node(n.right.take(), succ_val);
                Some(n)
            }
        }
    }
}

fn collect(root: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
    let Some(n) = root else { return };
    collect(&n.left, out);
    out.push(n.val);
    collect(&n.right, out);
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{collect, delete_node, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Some(Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode {
                val: 3,
                left: Some(Box::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
            })),
        }));
        root = delete_node(root, 3);
        let mut v = vec![];
        collect(&root, &mut v);
        assert_eq!(v, vec![2, 4, 5, 6, 7]);
    }
}
