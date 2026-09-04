/// LeetCode #655 - Print Binary Tree
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

fn print_tree(root: Option<Box<TreeNode>>) -> Vec<Vec<String>> {
    fn height(node: &Option<Box<TreeNode>>) -> usize {
        match node {
            None => 0,
            Some(n) => 1 + height(&n.left).max(height(&n.right)),
        }
    }
    let h = height(&root);
    let w = (1usize << h) - 1;
    let mut res = vec![vec![String::new(); w]; h];
    fn fill(
        node: &Option<Box<TreeNode>>,
        res: &mut Vec<Vec<String>>,
        r: usize,
        c: usize,
        h: usize,
    ) {
        let Some(n) = node else { return };
        res[r][c] = n.val.to_string();
        if r + 1 >= h {
            return;
        }
        let offset = 1 << (h - r - 2);
        fill(&n.left, res, r + 1, c - offset, h);
        fill(&n.right, res, r + 1, c + offset, h);
    }
    if h > 0 {
        fill(&root, &mut res, 0, (w - 1) / 2, h);
    }
    res
}

fn main() {
    let mut root = Box::new(TreeNode::new(1));
    root.left = Some(Box::new(TreeNode::new(2)));
    println!("{:?}", print_tree(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{print_tree, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(2)));
        let expected: Vec<Vec<String>> = vec![
            vec!["".into(), "1".into(), "".into()],
            vec!["2".into(), "".into(), "".into()],
        ];
        assert_eq!(print_tree(Some(root)), expected);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        let mut l = Box::new(TreeNode::new(2));
        l.right = Some(Box::new(TreeNode::new(4)));
        root.left = Some(l);
        root.right = Some(Box::new(TreeNode::new(3)));
        let expected: Vec<Vec<String>> = vec![
            vec![
                "".into(),
                "".into(),
                "".into(),
                "1".into(),
                "".into(),
                "".into(),
                "".into(),
            ],
            vec![
                "".into(),
                "2".into(),
                "".into(),
                "".into(),
                "".into(),
                "3".into(),
                "".into(),
            ],
            vec![
                "".into(),
                "".into(),
                "4".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
            ],
        ];
        assert_eq!(print_tree(Some(root)), expected);
    }
}
