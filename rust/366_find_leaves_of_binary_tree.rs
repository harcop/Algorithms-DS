/// LeetCode #366 - Find Leaves of Binary Tree
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn find_leaves(mut root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    fn dfs(o: &mut Option<Box<TreeNode>>, ans: &mut Vec<Vec<i32>>) -> isize {
        match o.as_mut() {
            None => -1,
            Some(n) => {
                let l = dfs(&mut n.left, ans);
                let r = dfs(&mut n.right, ans);
                let h = l.max(r) + 1;
                let hi = h as usize;
                if ans.len() <= hi {
                    ans.resize_with(hi + 1, Vec::new);
                }
                ans[hi].push(n.val);
                h
            }
        }
    }
    dfs(&mut root, &mut ans);
    ans
}

fn main() {
    println!("{}", find_leaves(None).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
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
        assert_eq!(find_leaves(root), vec![vec![2, 3], vec![1]]);
    }
}
