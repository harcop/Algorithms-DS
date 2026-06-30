/// LeetCode #2181 - Merge Nodes in Between Zeros
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode { val: 0, next: None };
    let mut tail = &mut dummy;
    let mut sum = 0i32;
    let mut cur = head;

    while let Some(node) = cur {
        if node.val == 0 && sum != 0 {
            tail.next = Some(Box::new(ListNode {
                val: sum,
                next: None,
            }));
            tail = tail.next.as_mut().unwrap();
            sum = 0;
        }
        sum += node.val;
        cur = node.next;
    }

    dummy.next
}

fn vec_to_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut ans = Vec::new();
    while let Some(node) = head {
        ans.push(node.val);
        head = node.next;
    }
    ans
}

fn main() {
    let head = vec_to_list(&[0, 3, 1, 0, 4, 5, 2, 0]);
    println!("{:?}", list_to_vec(merge_nodes(head)));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, merge_nodes, vec_to_list};

    #[test]
    fn example_one() {
        let head = vec_to_list(&[0, 3, 1, 0, 4, 5, 2, 0]);
        assert_eq!(list_to_vec(merge_nodes(head)), vec![4, 11]);
    }

    #[test]
    fn example_two() {
        let head = vec_to_list(&[0, 1, 0, 3, 0, 2, 2, 0]);
        assert_eq!(list_to_vec(merge_nodes(head)), vec![1, 3, 4]);
    }
}
