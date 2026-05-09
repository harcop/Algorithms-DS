/// LeetCode #543 - Diameter of Binary Tree
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn diameter_of_binary_tree(root: Option<Box<TreeNode>>) -> i32 {
    let mut best = 0i32;
    fn height(node: &Option<Box<TreeNode>>, best: &mut i32) -> i32 {
        let Some(n) = node else { return 0 };
        let l = height(&n.left, best);
        let r = height(&n.right, best);
        *best = (*best).max(l + r);
        l.max(r) + 1
    }
    height(&root, &mut best);
    best
}

fn main() {
    println!("{}", diameter_of_binary_tree(None));
}

#[cfg(test)]
mod tests {
    use super::{diameter_of_binary_tree, TreeNode};

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
        }));
        assert_eq!(diameter_of_binary_tree(root), 3);
    }
}
