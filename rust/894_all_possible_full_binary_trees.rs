/// LeetCode #894 - All Possible Full Binary Trees
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn all_possible_fbt(n: i32) -> Vec<Option<Box<TreeNode>>> {
    fn rec(
        n: usize,
        memo: &mut HashMap<usize, Vec<Option<Box<TreeNode>>>>,
    ) -> Vec<Option<Box<TreeNode>>> {
        if let Some(v) = memo.get(&n) {
            return v.clone();
        }
        if n == 1 {
            let out = vec![Some(Box::new(TreeNode::new(0)))];
            memo.insert(n, out.clone());
            return out;
        }
        let mut out = Vec::new();
        for i in (1..n).step_by(2) {
            let j = n - 1 - i;
            let lefts = rec(i, memo);
            let rights = rec(j, memo);
            for l in &lefts {
                for r in &rights {
                    let mut root = Box::new(TreeNode::new(0));
                    root.left = l.clone();
                    root.right = r.clone();
                    out.push(Some(root));
                }
            }
        }
        memo.insert(n, out.clone());
        out
    }
    let mut memo = HashMap::new();
    rec(n as usize, &mut memo)
}

fn main() {
    println!("{}", all_possible_fbt(7).len());
}

#[cfg(test)]
mod tests {
    use super::all_possible_fbt;

    #[test]
    fn example_one() {
        assert_eq!(all_possible_fbt(7).len(), 5);
    }
}
