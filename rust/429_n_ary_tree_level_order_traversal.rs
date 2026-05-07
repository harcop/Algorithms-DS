/// LeetCode #429 - N-ary Tree Level Order Traversal
use std::collections::VecDeque;

#[derive(Clone)]
pub struct NaryNode {
    pub val: i32,
    pub children: Vec<NaryNode>,
}

fn level_order(root: Option<Box<NaryNode>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut q = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    while !q.is_empty() {
        let sz = q.len();
        let mut row = Vec::with_capacity(sz);
        for _ in 0..sz {
            let node = q.pop_front().unwrap();
            row.push(node.val);
            for ch in node.children {
                q.push_back(Box::new(ch));
            }
        }
        ans.push(row);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        level_order(Some(Box::new(NaryNode {
            val: 1,
            children: vec![
                NaryNode {
                    val: 3,
                    children: vec![
                        NaryNode { val: 5, children: vec![] },
                        NaryNode { val: 6, children: vec![] },
                    ],
                },
                NaryNode { val: 2, children: vec![] },
                NaryNode { val: 4, children: vec![] },
            ],
        })))
    );
}

#[cfg(test)]
mod tests {
    use super::{level_order, NaryNode};

    #[test]
    fn example_one() {
        let root = NaryNode {
            val: 1,
            children: vec![
                NaryNode {
                    val: 3,
                    children: vec![
                        NaryNode { val: 5, children: vec![] },
                        NaryNode { val: 6, children: vec![] },
                    ],
                },
                NaryNode { val: 2, children: vec![] },
                NaryNode { val: 4, children: vec![] },
            ],
        };
        assert_eq!(
            level_order(Some(Box::new(root))),
            vec![vec![1], vec![3, 2, 4], vec![5, 6]]
        );
    }
}
