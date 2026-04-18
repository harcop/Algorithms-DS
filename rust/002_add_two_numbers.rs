/// LeetCode #2 - Add Two Numbers
///
/// You are given two non-empty linked lists representing two non-negative integers.
/// The digits are stored in reverse order, and each node contains a single digit.
/// Add the two numbers and return the sum as a linked list.

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

fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut tail = &mut dummy;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }

        carry = sum / 10;
        tail.next = Some(Box::new(ListNode::new(sum % 10)));
        tail = tail.next.as_mut().unwrap();
    }

    dummy.next
}

fn vec_to_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in values.iter().rev() {
        head = Some(Box::new(ListNode {
            val: v,
            next: head,
        }));
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
    let l1 = vec_to_list(&[2, 4, 3]);
    let l2 = vec_to_list(&[5, 6, 4]);
    let sum = add_two_numbers(l1, l2);
    println!("{:?}", list_to_vec(sum));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let l1 = vec_to_list(&[2, 4, 3]);
        let l2 = vec_to_list(&[5, 6, 4]);
        assert_eq!(list_to_vec(add_two_numbers(l1, l2)), vec![7, 0, 8]);
    }

    #[test]
    fn example_two() {
        let l1 = vec_to_list(&[0]);
        let l2 = vec_to_list(&[0]);
        assert_eq!(list_to_vec(add_two_numbers(l1, l2)), vec![0]);
    }

    #[test]
    fn example_three() {
        let l1 = vec_to_list(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = vec_to_list(&[9, 9, 9, 9]);
        assert_eq!(
            list_to_vec(add_two_numbers(l1, l2)),
            vec![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }
}
