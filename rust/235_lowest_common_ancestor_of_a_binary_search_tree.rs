/// LeetCode #235 - Lowest Common Ancestor of a Binary Search Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn lowest_common_ancestor(
    mut root: Option<Box<TreeNode>>,
    p: Option<Box<TreeNode>>,
    q: Option<Box<TreeNode>>,
) -> Option<Box<TreeNode>> {
    let pv = p.as_ref().unwrap().val;
    let qv = q.as_ref().unwrap().val;
    let (lo, hi) = if pv < qv { (pv, qv) } else { (qv, pv) };
    loop {
        let r = root.take().unwrap();
        let v = r.val;
        if v < lo {
            root = r.right;
        } else if v > hi {
            root = r.left;
        } else {
            return Some(r);
        }
    }
}

fn main() {
    println!("{:?}", lowest_common_ancestor(None, None, None));
}

#[cfg(test)]
mod tests {
    use super::{lowest_common_ancestor, TreeNode};

    #[test]
    fn example_one() {
        let p = Box::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        });
        let q = Box::new(TreeNode {
            val: 8,
            left: None,
            right: None,
        });
        let root = Box::new(TreeNode {
            val: 6,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 4,
                    left: Some(Box::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 8,
                left: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                })),
            })),
        });
        let ans = lowest_common_ancestor(Some(root), Some(p), Some(q));
        assert_eq!(ans.unwrap().val, 6);
    }
}
