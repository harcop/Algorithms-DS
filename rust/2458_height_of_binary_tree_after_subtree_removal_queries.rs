/// LeetCode #2458 - Height of Binary Tree After Subtree Removal Queries
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

fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
    const LIMIT: usize = 100_001;

    let root = root.unwrap();
    let mut depths = vec![0i32; LIMIT];
    let mut heights = vec![0i32; LIMIT];
    let mut best = vec![[(-1i32, 0usize); 2]; LIMIT];
    let mut stack = vec![(root, 0i32, false)];

    while let Some((node, depth, processed)) = stack.pop() {
        let (value, left, right) = {
            let node = node.borrow();
            (node.val as usize, node.left.clone(), node.right.clone())
        };

        if !processed {
            depths[value] = depth;
            stack.push((node, depth, true));
            if let Some(right) = right {
                stack.push((right, depth + 1, false));
            }
            if let Some(left) = left {
                stack.push((left, depth + 1, false));
            }
            continue;
        }

        let left_height = left
            .as_ref()
            .map_or(-1, |child| heights[child.borrow().val as usize]);
        let right_height = right
            .as_ref()
            .map_or(-1, |child| heights[child.borrow().val as usize]);
        let height = 1 + left_height.max(right_height);
        heights[value] = height;

        let level = &mut best[depth as usize];
        if height > level[0].0 {
            level[1] = level[0];
            level[0] = (height, value);
        } else if height > level[1].0 {
            level[1] = (height, value);
        }
    }

    queries
        .into_iter()
        .map(|query| {
            let node = query as usize;
            let depth = depths[node];
            let level = best[depth as usize];
            let remaining_height = if level[0].1 == node {
                level[1].0
            } else {
                level[0].0
            };
            depth + remaining_height
        })
        .collect()
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("{:?}", tree_queries(root, vec![]));
}

#[cfg(test)]
mod tests {
    use super::{tree_queries, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(value: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(value)))
    }

    #[test]
    fn removes_subtrees_at_different_depths() {
        let root = node(1);
        let left = node(2);
        let right = node(3);
        left.borrow_mut().left = Some(node(4));
        left.borrow_mut().right = Some(node(5));
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        assert_eq!(tree_queries(Some(root), vec![2, 3, 4]), vec![1, 2, 2]);
    }

    #[test]
    fn removes_only_node_at_its_depth() {
        let root = node(1);
        root.borrow_mut().left = Some(node(2));
        assert_eq!(tree_queries(Some(root), vec![2]), vec![0]);
    }
}
