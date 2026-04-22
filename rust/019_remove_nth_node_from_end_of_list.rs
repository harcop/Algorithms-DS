/// LeetCode #19 - Remove Nth Node From End of List
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

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut values = Vec::new();
    let mut curr = head;
    while let Some(node) = curr {
        values.push(node.val);
        curr = node.next;
    }

    let remove_idx = values.len() - n as usize;
    values.remove(remove_idx);
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
    let out = remove_nth_from_end(head, 2);
    println!("{:?}", list_to_vec(out));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, remove_nth_from_end, vec_to_list};

    #[test]
    fn example_one() {
        let head = vec_to_list(&[1, 2, 3, 4, 5]);
        assert_eq!(list_to_vec(remove_nth_from_end(head, 2)), vec![1, 2, 3, 5]);
    }

    #[test]
    fn example_two() {
        let head = vec_to_list(&[1]);
        assert!(list_to_vec(remove_nth_from_end(head, 1)).is_empty());
    }

    #[test]
    fn example_three() {
        let head = vec_to_list(&[1, 2]);
        assert_eq!(list_to_vec(remove_nth_from_end(head, 1)), vec![1]);
    }
}
