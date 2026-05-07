/// LeetCode #404 - Sum of Left Leaves
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn sum_of_left_leaves(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(o: &Option<Box<TreeNode>>, is_left: bool) -> i32 {
        match o.as_ref() {
            None => 0,
            Some(n) if n.left.is_none() && n.right.is_none() && is_left => n.val,
            Some(n) => dfs(&n.left, true) + dfs(&n.right, false),
        }
    }
    dfs(&root, false)
}

fn main() {
    println!("{}", sum_of_left_leaves(None));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 20,
                left: Some(Box::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
            })),
        }));
        assert_eq!(sum_of_left_leaves(root), 24);
    }
}
