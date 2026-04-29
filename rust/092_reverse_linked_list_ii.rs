/// LeetCode #92 - Reverse Linked List II
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

fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    if left == right {
        return head;
    }
    let mut vals = list_to_vec(head);
    let l = (left - 1) as usize;
    let r = (right - 1) as usize;
    vals[l..=r].reverse();
    vec_to_list(&vals)
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
    println!("{:?}", list_to_vec(reverse_between(vec_to_list(&[1, 2, 3, 4, 5]), 2, 4)));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, reverse_between, vec_to_list};

    #[test]
    fn example_one() {
        assert_eq!(
            list_to_vec(reverse_between(vec_to_list(&[1, 2, 3, 4, 5]), 2, 4)),
            vec![1, 4, 3, 2, 5]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            list_to_vec(reverse_between(vec_to_list(&[5]), 1, 1)),
            vec![5]
        );
    }
}
