/// LeetCode #24 - Swap Nodes in Pairs
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

fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut values = list_to_vec(head);
    let mut i = 0usize;
    while i + 1 < values.len() {
        values.swap(i, i + 1);
        i += 2;
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
    let head = vec_to_list(&[1, 2, 3, 4]);
    println!("{:?}", list_to_vec(swap_pairs(head)));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, swap_pairs, vec_to_list};

    #[test]
    fn example_one() {
        let head = vec_to_list(&[1, 2, 3, 4]);
        assert_eq!(list_to_vec(swap_pairs(head)), vec![2, 1, 4, 3]);
    }

    #[test]
    fn example_two() {
        assert!(list_to_vec(swap_pairs(None)).is_empty());
    }

    #[test]
    fn example_three() {
        let head = vec_to_list(&[1]);
        assert_eq!(list_to_vec(swap_pairs(head)), vec![1]);
    }
}
