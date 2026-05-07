/// LeetCode #328 - Odd Even Linked List
#[derive(Debug, PartialEq, Eq, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut head = head;
    let mut odd_tail = head.as_mut().unwrap();
    let mut even_head = odd_tail.next.take();
    if even_head.is_none() {
        return head;
    }
    let mut even_tail = even_head.as_mut().unwrap();
    loop {
        odd_tail.next = even_tail.next.take();
        if odd_tail.next.is_none() {
            break;
        }
        odd_tail = odd_tail.next.as_mut().unwrap();
        even_tail.next = odd_tail.next.take();
        if even_tail.next.is_none() {
            break;
        }
        even_tail = even_tail.next.as_mut().unwrap();
    }
    odd_tail.next = even_head;
    head
}

fn main() {
    let mut n3 = Box::new(ListNode::new(3));
    let mut n2 = Box::new(ListNode::new(2));
    n2.next = Some(n3);
    let mut n1 = Box::new(ListNode::new(1));
    n1.next = Some(n2);
    let _ = odd_even_list(Some(n1));
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::{odd_even_list, ListNode};

    fn to_vec(mut h: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = vec![];
        while let Some(n) = h {
            v.push(n.val);
            h = n.next;
        }
        v
    }

    fn from_vec(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut next = None;
        for &x in vals.iter().rev() {
            let mut n = Box::new(ListNode::new(x));
            n.next = next;
            next = Some(n);
        }
        next
    }

    #[test]
    fn example() {
        let h = from_vec(&[1, 2, 3, 4, 5]);
        let out = odd_even_list(h);
        assert_eq!(to_vec(out), vec![1, 3, 5, 2, 4]);
    }
}
