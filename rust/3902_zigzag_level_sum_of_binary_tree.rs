/// LeetCode #3902 - Zigzag Level Sum of Binary Tree
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn zigzag_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let Some(root) = root else {
        return vec![];
    };
    let mut ans = vec![];
    let mut q = VecDeque::from([root]);
    let mut depth = 0usize;
    while !q.is_empty() {
        let n = q.len();
        let mut level: Vec<Rc<RefCell<TreeNode>>> = Vec::with_capacity(n);
        for _ in 0..n {
            level.push(q.pop_front().unwrap());
        }
        let order: Vec<_> = if depth % 2 == 0 {
            level.iter().collect()
        } else {
            level.iter().rev().collect()
        };
        let mut sum = 0i32;
        for node in order {
            let node = node.borrow();
            if depth % 2 == 0 {
                if node.left.is_none() {
                    break;
                }
            } else if node.right.is_none() {
                break;
            }
            sum += node.val;
        }
        ans.push(sum);
        for node in level {
            let node = node.borrow();
            if let Some(l) = node.left.clone() {
                q.push_back(l);
            }
            if let Some(r) = node.right.clone() {
                q.push_back(r);
            }
        }
        depth += 1;
    }
    ans
}

fn main() {
    println!("{:?}", zigzag_level_sum(None));
}

#[cfg(test)]
mod tests {
    use super::{zigzag_level_sum, TreeNode};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    fn build(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode {
            val: vals[0].unwrap(),
            left: None,
            right: None,
        }));
        let mut q = VecDeque::from([root.clone()]);
        let mut i = 1usize;
        while i < vals.len() && !q.is_empty() {
            let cur = q.pop_front().unwrap();
            let mut cur = cur.borrow_mut();
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let child = Rc::new(RefCell::new(TreeNode {
                        val: v,
                        left: None,
                        right: None,
                    }));
                    cur.left = Some(child.clone());
                    q.push_back(child);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let child = Rc::new(RefCell::new(TreeNode {
                        val: v,
                        left: None,
                        right: None,
                    }));
                    cur.right = Some(child.clone());
                    q.push_back(child);
                }
                i += 1;
            }
        }
        Some(root)
    }

    #[test]
    fn example1() {
        let root = build(vec![
            Some(5),
            Some(2),
            Some(8),
            Some(1),
            None,
            Some(9),
            Some(6),
        ]);
        assert_eq!(zigzag_level_sum(root), vec![5, 8, 0]);
    }

    #[test]
    fn example2() {
        let root = build(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(7),
        ]);
        assert_eq!(zigzag_level_sum(root), vec![1, 5, 0]);
    }
}
