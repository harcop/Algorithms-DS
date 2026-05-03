/// LeetCode #206 - Reverse Linked List
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut cur = head;
    while let Some(mut node) = cur {
        cur = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

fn main() {
    println!("{:?}", reverse_list(None));
}

#[cfg(test)]
mod tests {
    use super::{reverse_list, ListNode};

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
        let h = list(&[1, 2, 3, 4, 5]);
        let out = reverse_list(h);
        assert_eq!(to_vec(&out), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn example_two() {
        let h = list(&[1, 2]);
        let out = reverse_list(h);
        assert_eq!(to_vec(&out), vec![2, 1]);
    }
}
