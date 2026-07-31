/// LeetCode #2816 - Double a Number Represented as a Linked List
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

fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vals = vec![];
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        vals.push(node.val);
        cur = node.next.as_ref();
    }

    let mut carry = 0;
    for v in vals.iter_mut().rev() {
        let doubled = *v * 2 + carry;
        *v = doubled % 10;
        carry = doubled / 10;
    }
    if carry > 0 {
        vals.insert(0, carry);
    }

    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
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

fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = vec![];
    let mut c = head.as_ref();
    while let Some(n) = c {
        v.push(n.val);
        c = n.next.as_ref();
    }
    v
}

fn main() {
    println!("{:?}", to_vec(&double_it(from_vec(&[1, 8, 9]))));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(to_vec(&double_it(from_vec(&[1, 8, 9]))), vec![3, 7, 8]);
    }

    #[test]
    fn example_two() {
        assert_eq!(to_vec(&double_it(from_vec(&[9, 9, 9]))), vec![1, 9, 9, 8]);
    }
}
