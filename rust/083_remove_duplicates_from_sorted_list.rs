/// LeetCode #83 - Remove Duplicates from Sorted List
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

fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head.as_mut();
    while let Some(node) = cur {
        while node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
            let next = node.next.take().unwrap().next;
            node.next = next;
        }
        cur = node.next.as_mut();
    }
    head
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
    println!("{:?}", list_to_vec(delete_duplicates(vec_to_list(&[1, 1, 2]))));
}

#[cfg(test)]
mod tests {
    use super::{delete_duplicates, list_to_vec, vec_to_list};

    #[test]
    fn example_one() {
        assert_eq!(
            list_to_vec(delete_duplicates(vec_to_list(&[1, 1, 2]))),
            vec![1, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            list_to_vec(delete_duplicates(vec_to_list(&[1, 1, 2, 3, 3]))),
            vec![1, 2, 3]
        );
    }
}
