/// LeetCode #725 - Split Linked List in Parts
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let k = k as usize;
    let mut len = 0usize;
    let mut p = head.as_ref();
    while let Some(n) = p {
        len += 1;
        p = n.next.as_ref();
    }
    let base = len / k;
    let extra = len % k;
    let mut cur = head;
    let mut res = vec![None; k];
    for i in 0..k {
        let size = base + if i < extra { 1 } else { 0 };
        if size == 0 {
            continue;
        }
        let mut dummy = ListNode::new(0);
        let mut w = &mut dummy.next;
        for _ in 0..size {
            let mut node = cur.take().unwrap();
            cur = node.next.take();
            *w = Some(node);
            w = &mut w.as_mut().unwrap().next;
        }
        res[i] = dummy.next.take();
    }
    res
}

fn main() {
    let mut n1 = Box::new(ListNode::new(1));
    n1.next = Some(Box::new(ListNode::new(2)));
    n1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    println!("{}", split_list_to_parts(Some(n1), 5).len());
}

#[cfg(test)]
mod tests {
    use super::{split_list_to_parts, ListNode};

    fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &x in v.iter().rev() {
            let mut n = Box::new(ListNode::new(x));
            n.next = head;
            head = Some(n);
        }
        head
    }

    fn to_vec(mut h: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = vec![];
        while let Some(n) = h {
            v.push(n.val);
            h = n.next;
        }
        v
    }

    #[test]
    fn example_one() {
        let parts = split_list_to_parts(from_vec(vec![1, 2, 3]), 5);
        assert_eq!(parts.len(), 5);
        assert_eq!(to_vec(parts[0].clone()), vec![1]);
        assert_eq!(to_vec(parts[1].clone()), vec![2]);
        assert_eq!(to_vec(parts[2].clone()), vec![3]);
    }
}
