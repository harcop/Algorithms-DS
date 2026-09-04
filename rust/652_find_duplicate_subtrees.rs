/// LeetCode #652 - Find Duplicate Subtrees
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

fn find_duplicate_subtrees(root: Option<Box<TreeNode>>) -> Vec<Option<Box<TreeNode>>> {
    let mut count: HashMap<String, i32> = HashMap::new();
    let mut ans = vec![];
    fn serial(
        node: &Option<Box<TreeNode>>,
        count: &mut HashMap<String, i32>,
        ans: &mut Vec<Option<Box<TreeNode>>>,
    ) -> String {
        let Some(n) = node else {
            return "#".into();
        };
        let s = format!(
            "{},{},{}",
            n.val,
            serial(&n.left, count, ans),
            serial(&n.right, count, ans)
        );
        let c = count.entry(s.clone()).or_insert(0);
        *c += 1;
        if *c == 2 {
            ans.push(Some(n.clone()));
        }
        s
    }
    serial(&root, &mut count, &mut ans);
    ans
}

fn serialize_tree(root: &Option<Box<TreeNode>>) -> String {
    match root {
        None => "#".into(),
        Some(n) => format!(
            "{},{},{}",
            n.val,
            serialize_tree(&n.left),
            serialize_tree(&n.right)
        ),
    }
}

fn main() {
    let mut root = Box::new(TreeNode::new(1));
    root.left = Some(Box::new(TreeNode::new(2)));
    println!("{:?}", find_duplicate_subtrees(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{find_duplicate_subtrees, serialize_tree, TreeNode};
    use std::collections::HashSet;

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(1));
        let mut l = Box::new(TreeNode::new(2));
        l.left = Some(Box::new(TreeNode::new(4)));
        let mut r = Box::new(TreeNode::new(3));
        let mut rl = Box::new(TreeNode::new(2));
        rl.left = Some(Box::new(TreeNode::new(4)));
        r.left = Some(rl);
        r.right = Some(Box::new(TreeNode::new(4)));
        root.left = Some(l);
        root.right = Some(r);
        let got: HashSet<String> = find_duplicate_subtrees(Some(root))
            .iter()
            .map(serialize_tree)
            .collect();
        let mut expected = HashSet::new();
        let mut t2 = Box::new(TreeNode::new(2));
        t2.left = Some(Box::new(TreeNode::new(4)));
        expected.insert(serialize_tree(&Some(t2)));
        expected.insert(serialize_tree(&Some(Box::new(TreeNode::new(4)))));
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(2));
        root.left = Some(Box::new(TreeNode::new(1)));
        root.right = Some(Box::new(TreeNode::new(1)));
        let got: HashSet<String> = find_duplicate_subtrees(Some(root))
            .iter()
            .map(serialize_tree)
            .collect();
        let mut expected = HashSet::new();
        expected.insert(serialize_tree(&Some(Box::new(TreeNode::new(1)))));
        assert_eq!(got, expected);
    }
}
