/// LeetCode #270 - Closest Binary Search Tree Value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn closest_value(root: Option<Box<TreeNode>>, target: f64) -> i32 {
    let mut best = root.as_ref().unwrap().val;
    let mut cur = root.as_ref();
    while let Some(n) = cur {
        if (n.val as f64 - target).abs() < (best as f64 - target).abs() {
            best = n.val;
        }
        if target < n.val as f64 {
            cur = n.left.as_ref();
        } else {
            cur = n.right.as_ref();
        }
    }
    best
}

fn main() {
    let root = Box::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    });
    println!("{}", closest_value(Some(root), 4.428571));
}

#[cfg(test)]
mod tests {
    use super::{closest_value, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 4,
            left: Some(Box::new(TreeNode {
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
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            })),
        });
        assert_eq!(closest_value(Some(root), 3.714286), 4);
    }
}
