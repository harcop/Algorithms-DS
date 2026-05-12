/// LeetCode #671 - Second Minimum Node In a Binary Tree
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn find_second_minimum_value(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(node: &Option<Box<TreeNode>>, min: i32, ans: &mut i64) {
        let Some(n) = node else { return };
        if (n.val as i64) > (min as i64) && (n.val as i64) < *ans {
            *ans = n.val as i64;
            return;
        }
        dfs(&n.left, min, ans);
        dfs(&n.right, min, ans);
    }
    let Some(r) = root.as_ref() else { return -1 };
    let mut ans: i64 = i64::MAX;
    dfs(&root, r.val, &mut ans);
    if ans == i64::MAX { -1 } else { ans as i32 }
}

fn main() {
    println!("{}", find_second_minimum_value(None));
}

#[cfg(test)]
mod tests {
    use super::{find_second_minimum_value, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode { val: 5, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 7, left: None, right: None })),
            })),
        }));
        assert_eq!(find_second_minimum_value(root), 5);
    }
}
