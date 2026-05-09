/// LeetCode #563 - Binary Tree Tilt
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn find_tilt(root: Option<Box<TreeNode>>) -> i32 {
    let mut tilt = 0i32;
    fn sum(node: &Option<Box<TreeNode>>, tilt: &mut i32) -> i32 {
        let Some(n) = node else { return 0 };
        let l = sum(&n.left, tilt);
        let r = sum(&n.right, tilt);
        *tilt += (l - r).abs();
        l + r + n.val
    }
    sum(&root, &mut tilt);
    tilt
}

fn main() {
    println!("{}", find_tilt(None));
}

#[cfg(test)]
mod tests {
    use super::{find_tilt, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
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
        }));
        assert_eq!(find_tilt(root), 1);
    }
}
