/// LeetCode #1214 - Two Sum BSTs
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn inorder(root: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
    if let Some(n) = root {
        inorder(&n.left, out);
        out.push(n.val);
        inorder(&n.right, out);
    }
}

fn two_sum_bs_ts(root1: Option<Box<TreeNode>>, root2: Option<Box<TreeNode>>, target: i32) -> bool {
    let mut a = Vec::new();
    let mut b = Vec::new();
    inorder(&root1, &mut a);
    inorder(&root2, &mut b);
    let mut i = 0usize;
    let mut j = b.len();
    while i < a.len() && j > 0 {
        let s = a[i] + b[j - 1];
        if s == target {
            return true;
        }
        if s < target {
            i += 1;
        } else {
            j -= 1;
        }
    }
    false
}

fn main() {
    println!("{}", two_sum_bs_ts(None, None, 0));
}

#[cfg(test)]
mod tests {
    use super::{two_sum_bs_ts, TreeNode};

    #[test]
    fn example_one() {
        let r1 = Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
        }));
        let r2 = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
        }));
        assert!(two_sum_bs_ts(r1, r2, 5));
    }

    #[test]
    fn example_two() {
        let r1 = Some(Box::new(TreeNode {
            val: 0,
            left: Some(Box::new(TreeNode {
                val: -10,
                left: None,
                right: None,
            })),
            right: None,
        }));
        let r2 = Some(Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                })),
            })),
        }));
        assert!(!two_sum_bs_ts(r1, r2, 18));
    }
}
