/// LeetCode #2487 - Remove Nodes From Linked List
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

fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let values = list_to_vec(head);
    let mut stack = Vec::new();
    for value in values {
        while stack.last().is_some_and(|&top| top < value) {
            stack.pop();
        }
        stack.push(value);
    }
    vec_to_list(&stack)
}

fn vec_to_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &value in values.iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = head;
        head = Some(node);
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut values = Vec::new();
    while let Some(node) = head {
        values.push(node.val);
        head = node.next;
    }
    values
}

fn main() {
    println!(
        "{:?}",
        list_to_vec(remove_nodes(vec_to_list(&[5, 2, 13, 3, 8])))
    );
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, remove_nodes, vec_to_list};

    #[test]
    fn example_one() {
        assert_eq!(
            list_to_vec(remove_nodes(vec_to_list(&[5, 2, 13, 3, 8]))),
            vec![13, 8]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            list_to_vec(remove_nodes(vec_to_list(&[1, 1, 1, 1]))),
            vec![1, 1, 1, 1]
        );
    }
}
