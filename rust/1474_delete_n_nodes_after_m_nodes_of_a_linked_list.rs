/// LeetCode #1474 - Delete N Nodes After M Nodes of a Linked List
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

fn delete_nodes(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: 0,
        next: head,
    });
    {
        let mut cur = dummy.as_mut();
        loop {
            let mut kept = 0;
            while kept < m && cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
                kept += 1;
            }
            if cur.next.is_none() {
                break;
            }
            let mut removed = 0;
            while removed < n && cur.next.is_some() {
                cur.next = cur.next.take().unwrap().next;
                removed += 1;
            }
            if cur.next.is_none() {
                break;
            }
        }
    }
    dummy.next
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
    let head = vec_to_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    println!("{:?}", list_to_vec(delete_nodes(head, 2, 3)));
}

#[cfg(test)]
mod tests {
    use super::{delete_nodes, list_to_vec, vec_to_list};

    #[test]
    fn example_one() {
        let head = vec_to_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
        assert_eq!(
            list_to_vec(delete_nodes(head, 2, 3)),
            vec![1, 2, 6, 7, 11, 12]
        );
    }

    #[test]
    fn example_two() {
        let head = vec_to_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(list_to_vec(delete_nodes(head, 1, 3)), vec![1, 5, 9]);
    }

    #[test]
    fn example_three() {
        let head = vec_to_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(
            list_to_vec(delete_nodes(head, 3, 1)),
            vec![1, 2, 3, 5, 6, 7, 9, 10, 11]
        );
    }
}
