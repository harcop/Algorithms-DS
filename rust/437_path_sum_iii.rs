/// LeetCode #437 - Path Sum III
use std::collections::HashMap;

#[derive(Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn path_sum(root: Option<Box<TreeNode>>, target_sum: i32) -> i32 {
    let mut pref = HashMap::new();
    pref.insert(0, 1);
    fn dfs(
        node: &Option<Box<TreeNode>>,
        mut sum: i64,
        target: i64,
        pref: &mut HashMap<i64, i32>,
    ) -> i32 {
        let Some(n) = node else { return 0 };
        sum += n.val as i64;
        let mut ans = *pref.get(&(sum - target)).unwrap_or(&0);
        *pref.entry(sum).or_insert(0) += 1;
        ans += dfs(&n.left, sum, target, pref);
        ans += dfs(&n.right, sum, target, pref);
        *pref.get_mut(&sum).unwrap() -= 1;
        ans
    }
    dfs(&root, 0, target_sum as i64, &mut pref)
}

fn main() {
    println!("{}", path_sum(None, 0));
}

#[cfg(test)]
mod tests {
    use super::{path_sum, TreeNode};

    #[test]
    fn smoke_none() {
        assert_eq!(path_sum(None, 1), 0);
    }

    #[test]
    fn smoke_single() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }));
        assert_eq!(path_sum(root, 1), 1);
    }
}
