/// LeetCode #2313 - Minimum Flips in Binary Tree to Get Result
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

fn minimum_flips(root: &Option<Box<TreeNode>>, result: bool) -> i32 {
    const INF: i32 = 1 << 30;

    fn dfs(root: &Option<Box<TreeNode>>) -> (i32, i32) {
        let Some(node) = root else {
            return (INF, INF);
        };
        let x = node.val;
        if x < 2 {
            return (x, x ^ 1);
        }
        let (l0, l1) = dfs(&node.left);
        let (r0, r1) = dfs(&node.right);
        match x {
            2 => (l0 + r0, (l0 + r1).min((l1 + r0).min(l1 + r1))),
            3 => ((l0 + r0).min((l0 + r1).min(l1 + r0)), l1 + r1),
            4 => ((l0 + r0).min(l1 + r1), (l0 + r1).min(l1 + r0)),
            _ => (l1.min(r1), l0.min(r0)),
        }
    }

    let (a, b) = dfs(root);
    if result { b } else { a }
}

fn main() {
    let mut root = Some(Box::new(TreeNode::new(3)));
    root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(5)));
    root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(4)));
    println!("{}", minimum_flips(&root, true));
}

#[cfg(test)]
mod tests {
    use super::{minimum_flips, TreeNode};

    fn example_tree() -> Option<Box<TreeNode>> {
        let mut n2 = TreeNode::new(2);
        n2.left = Some(Box::new(TreeNode::new(1)));
        n2.right = Some(Box::new(TreeNode::new(0)));

        let mut n5 = TreeNode::new(5);
        n5.left = Some(Box::new(n2));

        let mut n4 = TreeNode::new(4);
        n4.left = Some(Box::new(TreeNode::new(1)));
        n4.right = Some(Box::new(TreeNode::new(1)));

        let mut root = TreeNode::new(3);
        root.left = Some(Box::new(n5));
        root.right = Some(Box::new(n4));
        Some(Box::new(root))
    }

    #[test]
    fn example_one() {
        assert_eq!(minimum_flips(&example_tree(), true), 2);
    }

    #[test]
    fn example_two() {
        let root = Some(Box::new(TreeNode::new(0)));
        assert_eq!(minimum_flips(&root, false), 0);
    }
}
