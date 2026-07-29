/// LeetCode #2792 - Count Nodes That Are Great Enough
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn push_k(heap: &mut BinaryHeap<i32>, x: i32, k: usize) {
    heap.push(x);
    while heap.len() > k {
        heap.pop();
    }
}

fn count_great_enough_nodes(root: Option<Box<TreeNode>>, k: usize) -> i32 {
    let mut ans = 0;
    fn dfs(node: Option<Box<TreeNode>>, k: usize, ans: &mut i32) -> BinaryHeap<i32> {
        let Some(node) = node else {
            return BinaryHeap::new();
        };
        let mut heap = dfs(node.left, k, ans);
        let right = dfs(node.right, k, ans);
        for x in right {
            push_k(&mut heap, x, k);
        }
        if heap.len() == k {
            if let Some(&top) = heap.peek() {
                if top < node.val {
                    *ans += 1;
                }
            }
        }
        push_k(&mut heap, node.val, k);
        heap
    }
    dfs(root, k, &mut ans);
    ans
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 7,
        left: Some(Box::new(TreeNode {
            val: 6,
            left: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
        })),
    }));
    println!("{}", count_great_enough_nodes(root, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 7,
            left: Some(Box::new(TreeNode {
                val: 6,
                left: Some(Box::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                })),
            })),
        }));
        assert_eq!(count_great_enough_nodes(root, 2), 3);
    }

    #[test]
    fn example_two() {
        let root = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
        }));
        assert_eq!(count_great_enough_nodes(root, 1), 0);
    }

    #[test]
    fn example_three() {
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
        }));
        assert_eq!(count_great_enough_nodes(root, 2), 1);
    }
}
