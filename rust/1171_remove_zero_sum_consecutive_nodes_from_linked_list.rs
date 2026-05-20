/// LeetCode #1171 - Remove Zero Sum Consecutive Nodes from Linked List
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    loop {
        let mut prefix = 0i32;
        let mut seen: HashMap<i32, *mut ListNode> = HashMap::new();
        seen.insert(0, dummy.as_mut() as *mut ListNode);
        let mut curr = dummy.as_mut() as *mut ListNode;
        let mut removed = false;
        while unsafe { (*curr).next.is_some() } {
            let next = unsafe { (*curr).next.as_mut().unwrap().as_mut() as *mut ListNode };
            prefix += unsafe { (*next).val };
            if let Some(&prev) = seen.get(&prefix) {
                unsafe {
                    (*prev).next = (*next).next.take();
                }
                removed = true;
                break;
            }
            seen.insert(prefix, next);
            curr = next;
        }
        if !removed {
            break;
        }
    }
    dummy.next
}

fn list_from(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(n) = head {
        out.push(n.val);
        head = n.next;
    }
    out
}

fn main() {
    let head = list_from(&[1, 2, -3, 3, 1]);
    println!("{:?}", list_to_vec(remove_zero_sum_sublists(head)));
}

#[cfg(test)]
mod tests {
    use super::{list_from, list_to_vec, remove_zero_sum_sublists};

    #[test]
    fn example_one() {
        assert_eq!(
            list_to_vec(remove_zero_sum_sublists(list_from(&[1, 2, -3, 3, 1]))),
            vec![3, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            list_to_vec(remove_zero_sum_sublists(list_from(&[1, 2, 3, -3, 4]))),
            vec![1, 2, 4]
        );
    }
}
