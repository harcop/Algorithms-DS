/// LeetCode #25 - Reverse Nodes in k-Group
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

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
        return head;
    }

    let mut values = list_to_vec(head);
    let k = k as usize;
    let mut i = 0usize;
    while i + k <= values.len() {
        values[i..i + k].reverse();
        i += k;
    }
    vec_to_list(&values)
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
    let head = vec_to_list(&[1, 2, 3, 4, 5]);
    println!("{:?}", list_to_vec(reverse_k_group(head, 2)));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, reverse_k_group, vec_to_list};

    #[test]
    fn example_one() {
        let head = vec_to_list(&[1, 2, 3, 4, 5]);
        assert_eq!(list_to_vec(reverse_k_group(head, 2)), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn example_two() {
        let head = vec_to_list(&[1, 2, 3, 4, 5]);
        assert_eq!(list_to_vec(reverse_k_group(head, 3)), vec![3, 2, 1, 4, 5]);
    }
}
