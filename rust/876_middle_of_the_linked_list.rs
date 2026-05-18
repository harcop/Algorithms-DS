/// LeetCode #876 - Middle of the Linked List
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;
    while let Some(nf) = fast {
        match &nf.next {
            None => break,
            Some(n2) => {
                fast = &n2.next;
                slow = &slow.as_ref().unwrap().next;
            }
        }
    }
    slow.as_ref().cloned()
}

fn main() {
    println!("{:?}", middle_node(None));
}

#[cfg(test)]
mod tests {
    use super::{middle_node, ListNode};

    fn list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut h = None;
        for &v in vals.iter().rev() {
            h = Some(Box::new(ListNode { val: v, next: h }));
        }
        h
    }

    fn collect(mut h: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = vec![];
        while let Some(n) = h {
            v.push(n.val);
            h = n.next;
        }
        v
    }

    #[test]
    fn example_one() {
        let head = list(&[1, 2, 3, 4, 5]);
        let mid = middle_node(head);
        assert_eq!(collect(mid), vec![3, 4, 5]);
    }
}
