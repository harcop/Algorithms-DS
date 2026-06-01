/// LeetCode #1644 - Lowest Common Ancestor Of A Binary Tree Ii
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn lowest_common_ancestor(root: &Option<Box<TreeNode>>, p: &Option<Box<TreeNode>>, q: &Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let p = p.as_ref()?.val;
    let q = q.as_ref()?.val;
    fn dfs(r: &Option<Box<TreeNode>>, p: i32, q: i32) -> (bool, bool, Option<i32>) {
        let Some(n) = r else { return (false, false, None); };
        let (lp, lq, ll) = dfs(&n.left, p, q);
        let (rp, rq, rl) = dfs(&n.right, p, q);
        let has_p = lp || rp || n.val == p;
        let has_q = lq || rq || n.val == q;
        if ll.is_some() { return (has_p, has_q, ll); }
        if rl.is_some() { return (has_p, has_q, rl); }
        if has_p && has_q { return (true, true, Some(n.val)); }
        (has_p, has_q, None)
    }
    let val = dfs(root, p, q).2?;
    Some(Box::new(TreeNode { val, left: None, right: None }))
}
fn main() { println!("{:?}", lowest_common_ancestor(&None, &None, &None)); }
#[cfg(test)]
mod tests {
    use super::{lowest_common_ancestor, TreeNode};
    fn build(vals: Vec<Option<i32>>) -> Option<Box<TreeNode>> {
        if vals.is_empty() || vals[0].is_none() { return None; }
        let mut nodes: Vec<Option<Box<TreeNode>>> = vals.iter().map(|&v| {
            v.map(|x| Box::new(TreeNode { val: x, left: None, right: None }))
        }).collect();
        for i in 0..nodes.len() {
            if nodes[i].is_none() { continue; }
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            if l < nodes.len() { nodes[i].as_mut().unwrap().left = nodes[l].take(); }
            if r < nodes.len() { nodes[i].as_mut().unwrap().right = nodes[r].take(); }
        }
        nodes.remove(0)
    }
    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode { val: 6, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
            })),
            right: Some(Box::new(TreeNode {
                val: 1,
                left: Some(Box::new(TreeNode { val: 0, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 8, left: None, right: None })),
            })),
        }));
        let p = Some(Box::new(TreeNode { val: 5, left: None, right: None }));
        let q = Some(Box::new(TreeNode { val: 1, left: None, right: None }));
        assert_eq!(lowest_common_ancestor(&root, &p, &q).unwrap().val, 3);
    }
}