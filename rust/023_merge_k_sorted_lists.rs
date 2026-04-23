use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// LeetCode #23 - Merge k Sorted Lists
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for mut list in lists {
        while let Some(node) = list {
            min_heap.push(Reverse(node.val));
            list = node.next;
        }
    }

    let mut values = Vec::new();
    while let Some(Reverse(v)) = min_heap.pop() {
        values.push(v);
    }

    vec_to_list(&values)
}

fn vec_to_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in values.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(node) = head {
        out.push(node.val);
        head = node.next;
    }
    out
}

fn main() {
    let lists = vec![
        vec_to_list(&[1, 4, 5]),
        vec_to_list(&[1, 3, 4]),
        vec_to_list(&[2, 6]),
    ];
    println!("{:?}", list_to_vec(merge_k_lists(lists)));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, merge_k_lists, vec_to_list};

    #[test]
    fn example_one() {
        let lists = vec![
            vec_to_list(&[1, 4, 5]),
            vec_to_list(&[1, 3, 4]),
            vec_to_list(&[2, 6]),
        ];
        assert_eq!(list_to_vec(merge_k_lists(lists)), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn example_two() {
        let lists: Vec<Option<Box<super::ListNode>>> = vec![];
        assert!(list_to_vec(merge_k_lists(lists)).is_empty());
    }

    #[test]
    fn example_three() {
        let lists = vec![None];
        assert!(list_to_vec(merge_k_lists(lists)).is_empty());
    }
}
