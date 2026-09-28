/// LeetCode #3997 - Count Dominant Nodes in a Binary Tree
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn build_tree(vals: Vec<Option<i32>>) -> Option<Box<TreeNode>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let mut nodes: Vec<Option<Box<TreeNode>>> = vals
        .into_iter()
        .map(|v| {
            v.map(|x| {
                Box::new(TreeNode {
                    val: x,
                    left: None,
                    right: None,
                })
            })
        })
        .collect();
    let n = nodes.len();
    for i in (0..n).rev() {
        if nodes[i].is_none() {
            continue;
        }
        let left = if 2 * i + 1 < n {
            nodes[2 * i + 1].take()
        } else {
            None
        };
        let right = if 2 * i + 2 < n {
            nodes[2 * i + 2].take()
        } else {
            None
        };
        let node = nodes[i].as_mut().unwrap();
        node.left = left;
        node.right = right;
    }
    nodes[0].take()
}

fn count_dominant_nodes(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(node: Option<&TreeNode>, ans: &mut i32) -> i32 {
        let Some(node) = node else {
            return i32::MIN;
        };
        let l = dfs(node.left.as_deref(), ans);
        let r = dfs(node.right.as_deref(), ans);
        let mx = l.max(r).max(node.val);
        if mx == node.val {
            *ans += 1;
        }
        mx
    }
    let mut ans = 0;
    dfs(root.as_deref(), &mut ans);
    ans
}

fn main() {
    let root = build_tree(vec![
        Some(5),
        Some(3),
        Some(8),
        Some(2),
        Some(4),
        Some(7),
        Some(1),
    ]);
    println!("{}", count_dominant_nodes(root));
}

#[cfg(test)]
mod tests {
    use super::{build_tree, count_dominant_nodes};

    #[test]
    fn example1() {
        let root = build_tree(vec![
            Some(5),
            Some(3),
            Some(8),
            Some(2),
            Some(4),
            Some(7),
            Some(1),
        ]);
        assert_eq!(count_dominant_nodes(root), 5);
    }

    #[test]
    fn example2() {
        let root = build_tree(vec![Some(1), Some(2), Some(3), Some(1), Some(2)]);
        assert_eq!(count_dominant_nodes(root), 4);
    }
}
