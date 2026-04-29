/// LeetCode #86 - Partition List
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut before = ListNode::new(0);
    let mut after = ListNode::new(0);
    let mut p1 = &mut before;
    let mut p2 = &mut after;
    let mut cur = head;

    while let Some(mut node) = cur {
        cur = node.next.take();
        if node.val < x {
            p1.next = Some(node);
            p1 = p1.next.as_mut().unwrap();
        } else {
            p2.next = Some(node);
            p2 = p2.next.as_mut().unwrap();
        }
    }
    p2.next = None;
    p1.next = after.next;
    before.next
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
    let h = vec_to_list(&[1, 4, 3, 2, 5, 2]);
    println!("{:?}", list_to_vec(partition(h, 3)));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, partition, vec_to_list};

    #[test]
    fn example_one() {
        let h = vec_to_list(&[1, 4, 3, 2, 5, 2]);
        assert_eq!(list_to_vec(partition(h, 3)), vec![1, 2, 2, 4, 3, 5]);
    }

    #[test]
    fn example_two() {
        let h = vec_to_list(&[2, 1]);
        assert_eq!(list_to_vec(partition(h, 2)), vec![1, 2]);
    }
}
