/// LeetCode #2074 - Reverse Nodes in Even Length Groups
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut values = list_to_vec(head);
    let mut start = 0usize;
    let mut group = 1usize;

    while start < values.len() {
        let end = (start + group).min(values.len());
        if (end - start) % 2 == 0 {
            values[start..end].reverse();
        }
        start = end;
        group += 1;
    }

    vec_to_list(&values)
}

fn vec_to_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in values.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
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
    let out = reverse_even_length_groups(vec_to_list(&[5, 2, 6, 3, 9, 1, 7, 3, 8, 4]));
    println!("{:?}", list_to_vec(out));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, reverse_even_length_groups, vec_to_list};

    #[test]
    fn example_one() {
        let head = vec_to_list(&[5, 2, 6, 3, 9, 1, 7, 3, 8, 4]);
        assert_eq!(
            list_to_vec(reverse_even_length_groups(head)),
            vec![5, 6, 2, 3, 9, 1, 4, 8, 3, 7]
        );
    }

    #[test]
    fn example_two() {
        let head = vec_to_list(&[1, 1, 0, 6]);
        assert_eq!(list_to_vec(reverse_even_length_groups(head)), vec![1, 0, 1, 6]);
    }

    #[test]
    fn example_three() {
        let head = vec_to_list(&[1, 1, 0, 6, 5]);
        assert_eq!(
            list_to_vec(reverse_even_length_groups(head)),
            vec![1, 0, 1, 5, 6]
        );
    }
}
