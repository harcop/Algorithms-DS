/// LeetCode #1836 - Remove Duplicates From an Unsorted Linked List
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

fn delete_duplicates_unsorted(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    use std::collections::HashMap;

    let mut freq: HashMap<i32, i32> = HashMap::new();
    let mut curr = head.as_deref();
    while let Some(node) = curr {
        *freq.entry(node.val).or_insert(0) += 1;
        curr = node.next.as_deref();
    }

    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    let mut curr = head;
    while let Some(mut node) = curr {
        curr = node.next.take();
        if freq.get(&node.val).copied().unwrap_or(0) == 1 {
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }
    }
    dummy.next
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
    let head = vec_to_list(&[1, 2, 3, 2]);
    println!("{:?}", list_to_vec(delete_duplicates_unsorted(head)));
}

#[cfg(test)]
mod tests {
    use super::{delete_duplicates_unsorted, list_to_vec, vec_to_list};

    #[test]
    fn example_one() {
        let head = vec_to_list(&[1, 2, 3, 2]);
        assert_eq!(list_to_vec(delete_duplicates_unsorted(head)), vec![1, 3]);
    }

    #[test]
    fn example_two() {
        let head = vec_to_list(&[2, 1, 1, 2]);
        assert!(list_to_vec(delete_duplicates_unsorted(head)).is_empty());
    }

    #[test]
    fn example_three() {
        let head = vec_to_list(&[3, 2, 2, 1, 3, 2, 4]);
        assert_eq!(
            list_to_vec(delete_duplicates_unsorted(head)),
            vec![1, 4]
        );
    }
}
