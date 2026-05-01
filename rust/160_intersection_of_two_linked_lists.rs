/// LeetCode #160 - Intersection of Two Linked Lists
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<ListNode>>,
}

fn len(head: &Option<Rc<ListNode>>) -> usize {
    let mut n = 0;
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        n += 1;
        cur = node.next.as_ref();
    }
    n
}

fn advance(head: Option<Rc<ListNode>>, k: usize) -> Option<Rc<ListNode>> {
    let mut cur = head;
    for _ in 0..k {
        cur = cur?.next.clone();
    }
    cur
}

fn get_intersection_node(
    head_a: Option<Rc<ListNode>>,
    head_b: Option<Rc<ListNode>>,
) -> Option<Rc<ListNode>> {
    let la = len(&head_a);
    let lb = len(&head_b);
    let mut pa = if la > lb {
        advance(head_a, la - lb)
    } else {
        head_a
    };
    let mut pb = if lb > la {
        advance(head_b, lb - la)
    } else {
        head_b
    };

    while let (Some(a), Some(b)) = (pa.as_ref(), pb.as_ref()) {
        if Rc::ptr_eq(a, b) {
            return Some(a.clone());
        }
        pa = a.next.clone();
        pb = b.next.clone();
    }
    None
}

/// Prepends `vals` (in list order) before `tail`.
fn prepend_vals(vals: &[i32], tail: Option<Rc<ListNode>>) -> Option<Rc<ListNode>> {
    let mut n = tail;
    for &v in vals.iter().rev() {
        n = Some(Rc::new(ListNode { val: v, next: n }));
    }
    n
}

fn main() {
    println!("{}", get_intersection_node(None, None).is_none());
}

#[cfg(test)]
mod tests {
    use super::{get_intersection_node, prepend_vals};

    #[test]
    fn intersect_at_8() {
        let common = prepend_vals(&[8, 4, 5], None);
        let a = prepend_vals(&[4, 1], common.clone());
        let b = prepend_vals(&[5, 6, 1], common);
        let got = get_intersection_node(a, b);
        assert_eq!(got.as_ref().map(|n| n.val), Some(8));
    }

    #[test]
    fn no_intersection() {
        let a = prepend_vals(&[1, 2, 3], None);
        let b = prepend_vals(&[4, 5], None);
        assert!(get_intersection_node(a, b).is_none());
    }
}
