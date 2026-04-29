/// LeetCode #113 - Path Sum II
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

fn path_sum(root: Option<Box<TreeNode>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    let mut path = Vec::new();
    fn dfs(
        node: &Option<Box<TreeNode>>,
        remain: i32,
        path: &mut Vec<i32>,
        out: &mut Vec<Vec<i32>>,
    ) {
        if let Some(n) = node {
            path.push(n.val);
            let remain = remain - n.val;
            if n.left.is_none() && n.right.is_none() {
                if remain == 0 {
                    out.push(path.clone());
                }
            } else {
                dfs(&n.left, remain, path, out);
                dfs(&n.right, remain, path, out);
            }
            path.pop();
        }
    }
    dfs(&root, target_sum, &mut path, &mut out);
    out
}

fn main() {
    println!("{:?}", path_sum(None, 0));
}

#[cfg(test)]
mod tests {
    use super::{path_sum, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(5));
        root.left = Some(Box::new(TreeNode::new(4)));
        root.right = Some(Box::new(TreeNode::new(8)));
        root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(11)));
        root.left.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(7)));
        root.left.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(2)));
        root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(13)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(4)));
        root.right.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(5)));
        root.right.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(1)));

        let mut got = path_sum(Some(root), 22);
        got.sort();
        let mut expected = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];
        expected.sort();
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(2)));
        assert!(path_sum(Some(root), 0).is_empty());
    }
}
