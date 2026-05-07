/// LeetCode #337 - House Robber III
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn rob(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(n: &Option<Box<TreeNode>>) -> (i32, i32) {
        // (best if this node robbed, best if this node skipped)
        match n {
            None => (0, 0),
            Some(node) => {
                let (lr, ln) = dfs(&node.left);
                let (rr, rn) = dfs(&node.right);
                let take = node.val + ln + rn;
                let skip = lr.max(ln) + rr.max(rn);
                (take, skip)
            }
        }
    }
    let (t, s) = dfs(&root);
    t.max(s)
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Box::new(TreeNode::leaf(3))),
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Box::new(TreeNode::leaf(1))),
        })),
    }));
    println!("{}", rob(root));
}

impl TreeNode {
    fn leaf(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{rob, TreeNode};

    #[test]
    fn leet_example1() {
        // [3,2,3,null,3,null,1]
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Box::new(TreeNode::leaf(3))),
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Box::new(TreeNode::leaf(1))),
            })),
        }));
        assert_eq!(rob(root), 7);
    }
}
