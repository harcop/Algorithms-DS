/// LeetCode #82 - Remove Duplicates from Sorted List II
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

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let vals = list_to_vec(head);
    let mut i = 0usize;
    let mut out = Vec::new();
    while i < vals.len() {
        let mut j = i + 1;
        while j < vals.len() && vals[j] == vals[i] {
            j += 1;
        }
        if j == i + 1 {
            out.push(vals[i]);
        }
        i = j;
    }
    vec_to_list(&out)
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
    println!(
        "{:?}",
        list_to_vec(delete_duplicates(vec_to_list(&[1, 2, 3, 3, 4, 4, 5])))
    );
}

#[cfg(test)]
mod tests {
    use super::{delete_duplicates, list_to_vec, vec_to_list};

    #[test]
    fn example_one() {
        assert_eq!(
            list_to_vec(delete_duplicates(vec_to_list(&[1, 2, 3, 3, 4, 4, 5]))),
            vec![1, 2, 5]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            list_to_vec(delete_duplicates(vec_to_list(&[1, 1, 1, 2, 3]))),
            vec![2, 3]
        );
    }
}
