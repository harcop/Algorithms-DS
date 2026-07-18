/// LeetCode #2476 - Closest Nodes Queries in a Binary Search Tree
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted = Vec::new();
    let mut stack = Vec::new();
    let mut current = root;

    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.borrow().left.clone();
        }
        let node = stack.pop().unwrap();
        sorted.push(node.borrow().val);
        current = node.borrow().right.clone();
    }

    queries
        .into_iter()
        .map(|query| {
            let index = sorted.partition_point(|&value| value < query);
            if index < sorted.len() && sorted[index] == query {
                vec![query, query]
            } else {
                vec![
                    if index == 0 { -1 } else { sorted[index - 1] },
                    if index == sorted.len() {
                        -1
                    } else {
                        sorted[index]
                    },
                ]
            }
        })
        .collect()
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    println!("{:?}", closest_nodes(root, vec![2, 5, 16]));
}

#[cfg(test)]
mod tests {
    use super::{closest_nodes, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(value: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(value)))
    }

    #[test]
    fn example_one() {
        let root = node(6);
        let left = node(2);
        let right = node(13);
        left.borrow_mut().left = Some(node(1));
        left.borrow_mut().right = Some(node(4));
        right.borrow_mut().left = Some(node(9));
        let fifteen = node(15);
        fifteen.borrow_mut().left = Some(node(14));
        right.borrow_mut().right = Some(fifteen);
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        assert_eq!(
            closest_nodes(Some(root), vec![2, 5, 16]),
            vec![vec![2, 2], vec![4, 6], vec![15, -1]]
        );
    }

    #[test]
    fn single_node() {
        let root = node(1);
        assert_eq!(
            closest_nodes(Some(root), vec![1, 0, 2]),
            vec![vec![1, 1], vec![-1, 1], vec![1, -1]]
        );
    }
}
