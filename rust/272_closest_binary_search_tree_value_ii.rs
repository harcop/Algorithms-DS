/// LeetCode #272 - Closest Binary Search Tree Value II
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn closest_k_values(root: Option<Box<TreeNode>>, target: f64, k: i32) -> Vec<i32> {
    if k == 0 {
        return vec![];
    }
    let mut a = vec![];
    fn dfs(n: &Option<Box<TreeNode>>, a: &mut Vec<i32>) {
        if let Some(b) = n {
            dfs(&b.left, a);
            a.push(b.val);
            dfs(&b.right, a);
        }
    }
    dfs(&root, &mut a);
    let mut r = a.partition_point(|&x| (x as f64) < target);
    let mut l = r as isize - 1;
    let mut out = vec![];
    for _ in 0..k {
        let pick_left = if r >= a.len() {
            true
        } else if l < 0 {
            false
        } else {
            (target - a[l as usize] as f64).abs() <= (a[r] as f64 - target).abs()
        };
        if pick_left {
            out.push(a[l as usize]);
            l -= 1;
        } else {
            out.push(a[r]);
            r += 1;
        }
    }
    out
}

fn main() {
    println!("{:?}", closest_k_values(None, 0.0, 0));
}

#[cfg(test)]
mod tests {
    use super::{closest_k_values, TreeNode};

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
                val: 6,
                left: Some(Box::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
            })),
        });
        let mut v = closest_k_values(Some(root), 3.8, 2);
        v.sort();
        assert_eq!(v, vec![3, 4]);
    }
}
