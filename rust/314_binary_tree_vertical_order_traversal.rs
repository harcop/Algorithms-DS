/// LeetCode #314 - Binary Tree Vertical Order Traversal
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn vertical_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    let mut map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    let mut q = VecDeque::new();
    if let Some(r) = root {
        q.push_back((r, 0i32));
    }
    while let Some((node, col)) = q.pop_front() {
        map.entry(col).or_default().push(node.val);
        if let Some(l) = node.left {
            q.push_back((l, col - 1));
        }
        if let Some(r) = node.right {
            q.push_back((r, col + 1));
        }
    }
    map.into_values().collect()
}

fn main() {
    println!("{:?}", vertical_order(None));
}

#[cfg(test)]
mod tests {
    use super::{vertical_order, TreeNode};

    #[test]
    fn example_one() {
        let root = Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 20,
                left: Some(Box::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
            })),
        });
        assert_eq!(
            vertical_order(Some(root)),
            vec![vec![9], vec![3, 15], vec![20], vec![7]]
        );
    }
}
