/// LeetCode #2807 - Insert Greatest Common Divisors in Linked List
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head.as_mut();
    while let Some(node) = cur {
        if let Some(next) = node.next.take() {
            let g = gcd(node.val, next.val);
            node.next = Some(Box::new(ListNode {
                val: g,
                next: Some(next),
            }));
            cur = node.next.as_mut().and_then(|gnode| gnode.next.as_mut());
        } else {
            break;
        }
    }
    head
}

fn from_vec(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
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

fn main() {
    println!("{:?}", to_vec(&insert_greatest_common_divisors(from_vec(&[18, 6, 10, 3]))));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            to_vec(&insert_greatest_common_divisors(from_vec(&[18, 6, 10, 3]))),
            vec![18, 6, 6, 2, 10, 1, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            to_vec(&insert_greatest_common_divisors(from_vec(&[7]))),
            vec![7]
        );
    }
}
