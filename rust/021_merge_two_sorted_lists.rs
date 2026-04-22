/// LeetCode #21 - Merge Two Sorted Lists
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

fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut values = Vec::new();

    while list1.is_some() && list2.is_some() {
        if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
            let node = list1.unwrap();
            values.push(node.val);
            list1 = node.next;
        } else {
            let node = list2.unwrap();
            values.push(node.val);
            list2 = node.next;
        }
    }

    while let Some(node) = list1 {
        values.push(node.val);
        list1 = node.next;
    }
    while let Some(node) = list2 {
        values.push(node.val);
        list2 = node.next;
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
    let l1 = vec_to_list(&[1, 2, 4]);
    let l2 = vec_to_list(&[1, 3, 4]);
    let merged = merge_two_lists(l1, l2);
    println!("{:?}", list_to_vec(merged));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, merge_two_lists, vec_to_list};

    #[test]
    fn example_one() {
        let l1 = vec_to_list(&[1, 2, 4]);
        let l2 = vec_to_list(&[1, 3, 4]);
        assert_eq!(list_to_vec(merge_two_lists(l1, l2)), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn example_two() {
        assert!(list_to_vec(merge_two_lists(None, None)).is_empty());
    }

    #[test]
    fn example_three() {
        let l2 = vec_to_list(&[0]);
        assert_eq!(list_to_vec(merge_two_lists(None, l2)), vec![0]);
    }
}
