/// LeetCode #2130 - Maximum Twin Sum of a Linked List
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

fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut values = Vec::new();
    let mut node = head.as_ref();

    while let Some(current) = node {
        values.push(current.val);
        node = current.next.as_ref();
    }

    let n = values.len();
    let mut best = 0;
    for i in 0..n / 2 {
        best = best.max(values[i] + values[n - 1 - i]);
    }
    best
}

fn main() {
    println!("{}", pair_sum(list(&[5, 4, 2, 1])));
}

fn list(values: &[i32]) -> Option<Box<ListNode>> {
    values.iter().rev().fold(None, |next, &val| {
        let mut node = Box::new(ListNode::new(val));
        node.next = next;
        Some(node)
    })
}

#[cfg(test)]
mod tests {
    use super::{list, pair_sum};

    #[test]
    fn example_one() {
        assert_eq!(pair_sum(list(&[5, 4, 2, 1])), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(pair_sum(list(&[4, 2, 2, 3])), 7);
    }

    #[test]
    fn example_three() {
        assert_eq!(pair_sum(list(&[1, 100000])), 100001);
    }
}
