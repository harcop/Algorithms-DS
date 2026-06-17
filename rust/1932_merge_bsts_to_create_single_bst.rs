/// LeetCode #1932 - Merge BSTs to Create Single BST
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn can_merge(trees: Vec<Option<Box<TreeNode>>>) -> Option<Box<TreeNode>> {
    let mut roots: HashMap<i32, Box<TreeNode>> = HashMap::new();
    let mut leaves = std::collections::HashSet::new();
    for tree in trees.into_iter().flatten() {
        if let Some(left) = &tree.left {
            leaves.insert(left.val);
        }
        if let Some(right) = &tree.right {
            leaves.insert(right.val);
        }
        roots.insert(tree.val, tree);
    }

    let mut ans = None;
    for val in roots.keys().copied().collect::<Vec<_>>() {
        if !leaves.contains(&val) {
            if ans.is_some() {
                return None;
            }
            ans = dfs(roots.remove(&val).unwrap(), &mut roots, i32::MIN, i32::MAX);
        }
    }

    if roots.is_empty() { ans } else { None }
}

fn is_leaf(node: &TreeNode) -> bool {
    node.left.is_none() && node.right.is_none()
}

fn dfs(
    mut node: Box<TreeNode>,
    roots: &mut HashMap<i32, Box<TreeNode>>,
    lo: i32,
    hi: i32,
) -> Option<Box<TreeNode>> {
    if node.val <= lo || node.val >= hi {
        return None;
    }

    node.left = match node.left.take() {
        None => None,
        Some(c) => {
            let next = if is_leaf(&c) && roots.contains_key(&c.val) {
                dfs(roots.remove(&c.val).unwrap(), roots, lo, node.val)
            } else {
                dfs(c, roots, lo, node.val)
            };
            match next {
                Some(t) => Some(t),
                None => return None,
            }
        }
    };
    node.right = match node.right.take() {
        None => None,
        Some(c) => {
            let next = if is_leaf(&c) && roots.contains_key(&c.val) {
                dfs(roots.remove(&c.val).unwrap(), roots, node.val, hi)
            } else {
                dfs(c, roots, node.val, hi)
            };
            match next {
                Some(t) => Some(t),
                None => return None,
            }
        }
    };

    roots.remove(&node.val);
    Some(node)
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    node(val, None, None)
}

fn main() {
    let t0 = node(2, leaf(1), None);
    let t1 = node(3, leaf(2), leaf(5));
    let t2 = node(5, leaf(4), None);
    println!("{:?}", can_merge(vec![t0, t1, t2]).map(|n| n.val));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let t0 = node(2, leaf(1), None);
        let t1 = node(3, leaf(2), leaf(5));
        let t2 = node(5, leaf(4), None);
        let ans = can_merge(vec![t0, t1, t2]).unwrap();
        assert_eq!(ans.val, 3);
        assert_eq!(ans.left.as_ref().unwrap().val, 2);
        assert_eq!(ans.right.as_ref().unwrap().val, 5);
        assert_eq!(ans.left.as_ref().unwrap().left.as_ref().unwrap().val, 1);
        assert_eq!(ans.right.as_ref().unwrap().left.as_ref().unwrap().val, 4);
    }

    #[test]
    fn example_two() {
        let t0 = node(5, leaf(3), leaf(8));
        let t1 = node(3, leaf(2), leaf(6));
        assert!(can_merge(vec![t0, t1]).is_none());
    }

    #[test]
    fn example_three() {
        let t0 = node(5, leaf(4), None);
        let t1 = leaf(3);
        assert!(can_merge(vec![t0, t1]).is_none());
    }
}
