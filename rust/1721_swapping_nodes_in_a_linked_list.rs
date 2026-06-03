/// LeetCode #1721 - Swapping Nodes in a Linked List
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

fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut vals = list_to_vec(head);
    if vals.is_empty() {
        return None;
    }
    let k = k as usize;
    let j = vals.len() - k;
    vals.swap(k - 1, j);
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
    let mut out = vec![];
    while let Some(node) = head {
        out.push(node.val);
        head = node.next;
    }
    out
}

fn main() {
    println!("{:?}", list_to_vec(swap_nodes(vec_to_list(&[1, 2, 3, 4, 5]), 2)));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_one() {
        assert_eq!(
            list_to_vec(swap_nodes(vec_to_list(&[1, 2, 3, 4, 5]), 2)),
            vec![1, 4, 3, 2, 5]
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(
            list_to_vec(swap_nodes(
                vec_to_list(&[7, 9, 6, 6, 7, 8, 3, 0, 9, 5]),
                5
            )),
            vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5]
        );
    }
}
