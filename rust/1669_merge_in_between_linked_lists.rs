/// LeetCode #1669 - Merge In Between Linked Lists
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn merge_in_between(list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: list1 });
    let mut cur = dummy.as_mut();
    for _ in 0..a { cur = cur.next.as_mut().unwrap(); }
    let mut tail = cur.next.as_mut().unwrap();
    for _ in 0..(b - a) { tail = tail.next.as_mut().unwrap(); }
    let rest = tail.next.take();
    cur.next = list2;
    while cur.next.is_some() { cur = cur.next.as_mut().unwrap(); }
    cur.next = rest;
    dummy.next
}
fn main() { println!("{:?}", merge_in_between(None, 0, 0, None)); }
#[cfg(test)]
mod tests {
    use super::{merge_in_between, ListNode};
    fn build(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &x in v.iter().rev() { head = Some(Box::new(ListNode { val: x, next: head })); }
        head
    }
    fn vals(mut h: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = vec![];
        while let Some(n) = h { out.push(n.val); h = n.next; }
        out
    }
    #[test]
    fn example_one() {
        let l1 = build(vec![10,1,13,6,9,5]);
        let l2 = build(vec![1000000,1000001,1000002]);
        let r = merge_in_between(l1, 3, 4, l2);
        assert_eq!(vals(r), vec![10,1,13,1000000,1000001,1000002,5]);
    }
}