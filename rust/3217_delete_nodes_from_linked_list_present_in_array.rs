/// LeetCode #3217 - Delete Nodes From Linked List Present in Array
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let s: HashSet<i32> = nums.into_iter().collect();
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut pre = &mut dummy;

    while let Some(ref mut node) = pre.next {
        if s.contains(&node.val) {
            pre.next = node.next.take();
        } else {
            pre = pre.next.as_mut().unwrap();
        }
    }

    dummy.next
}

fn from_vec(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
}

fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = vec![];
    let mut c = head;
    while let Some(n) = c {
        v.push(n.val);
        c = n.next;
    }
    v
}

fn main() {
    println!(
        "{:?}",
        to_vec(modified_list(vec![1, 2, 3], from_vec(&[1, 2, 3, 4, 5])))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            to_vec(modified_list(vec![1, 2, 3], from_vec(&[1, 2, 3, 4, 5]))),
            vec![4, 5]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            to_vec(modified_list(vec![1], from_vec(&[1, 2, 1, 2, 1, 2]))),
            vec![2, 2, 2]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            to_vec(modified_list(vec![5], from_vec(&[1, 2, 3, 4]))),
            vec![1, 2, 3, 4]
        );
    }
}
