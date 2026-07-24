/// LeetCode #2641 - Cousins in Binary Tree II
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

fn replace_value_in_tree(mut root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let mut s = Vec::new();

    fn dfs1(node: &TreeNode, depth: usize, s: &mut Vec<i32>) {
        if s.len() <= depth {
            s.push(0);
        }
        s[depth] += node.val;
        if let Some(l) = &node.left {
            dfs1(l, depth + 1, s);
        }
        if let Some(r) = &node.right {
            dfs1(r, depth + 1, s);
        }
    }

    fn dfs2(node: &mut TreeNode, depth: usize, s: &[i32]) {
        let sub = node.left.as_ref().map(|x| x.val).unwrap_or(0)
            + node.right.as_ref().map(|x| x.val).unwrap_or(0);
        let depth = depth + 1;
        if let Some(l) = node.left.as_mut() {
            l.val = s[depth] - sub;
            dfs2(l, depth, s);
        }
        if let Some(r) = node.right.as_mut() {
            r.val = s[depth] - sub;
            dfs2(r, depth, s);
        }
    }

    if let Some(r) = root.as_mut() {
        dfs1(r, 0, &mut s);
        r.val = 0;
        dfs2(r, 0, &s);
    }
    root
}

fn build(vals: &[Option<i32>]) -> Option<Box<TreeNode>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let mut nodes: Vec<Option<Box<TreeNode>>> = vals
        .iter()
        .map(|v| v.map(|x| Box::new(TreeNode::new(x))))
        .collect();
    let n = nodes.len();
    for i in (0..n).rev() {
        if nodes[i].is_none() {
            continue;
        }
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        let left = if l < n { nodes[l].take() } else { None };
        let right = if r < n { nodes[r].take() } else { None };
        let node = nodes[i].as_mut().unwrap();
        node.left = left;
        node.right = right;
    }
    nodes[0].take()
}

fn to_level_order(root: &Option<Box<TreeNode>>) -> Vec<Option<i32>> {
    let mut out = Vec::new();
    let mut q = vec![root.as_ref()];
    while !q.is_empty() {
        let mut next = Vec::new();
        let mut any = false;
        for n in &q {
            match n {
                None => out.push(None),
                Some(node) => {
                    out.push(Some(node.val));
                    next.push(node.left.as_ref());
                    next.push(node.right.as_ref());
                    if node.left.is_some() || node.right.is_some() {
                        any = true;
                    }
                }
            }
        }
        if !any {
            break;
        }
        q = next;
    }
    while out.last() == Some(&None) {
        out.pop();
    }
    out
}

fn main() {
    let root = build(&[
        Some(5),
        Some(4),
        Some(9),
        Some(1),
        Some(10),
        None,
        Some(7),
    ]);
    let ans = replace_value_in_tree(root);
    println!("{:?}", to_level_order(&ans));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = build(&[
            Some(5),
            Some(4),
            Some(9),
            Some(1),
            Some(10),
            None,
            Some(7),
        ]);
        let ans = replace_value_in_tree(root);
        assert_eq!(
            to_level_order(&ans),
            vec![Some(0), Some(0), Some(0), Some(7), Some(7), None, Some(11)]
        );
    }

    #[test]
    fn example_two() {
        let root = build(&[Some(3), Some(1), Some(2)]);
        let ans = replace_value_in_tree(root);
        assert_eq!(to_level_order(&ans), vec![Some(0), Some(0), Some(0)]);
    }
}
