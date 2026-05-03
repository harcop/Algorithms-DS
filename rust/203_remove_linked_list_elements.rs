/// LeetCode #203 - Remove Linked List Elements
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut dummy;
    let mut cur = head;
    while let Some(mut node) = cur {
        cur = node.next.take();
        if node.val != val {
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }
    }
    dummy.next
}

fn main() {
    println!("{:?}", remove_elements(None, 1));
}

#[cfg(test)]
mod tests {
    use super::{remove_elements, ListNode};

    fn list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut h = None;
        for &v in vals.iter().rev() {
            h = Some(Box::new(ListNode { val: v, next: h }));
        }
        h
    }

    fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = vec![];
        let mut c = head.as_ref();
        while let Some(n) = c {
            v.push(n.val);
            c = n.next.as_ref();
        }
        v
    }

    #[test]
    fn example_one() {
        let h = list(&[1, 2, 6, 3, 4, 5, 6]);
        let out = remove_elements(h, 6);
        assert_eq!(to_vec(&out), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn example_two() {
        assert!(remove_elements(None, 1).is_none());
    }
}
