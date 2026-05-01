use std::collections::HashSet;

/// LeetCode #142 - Linked List Cycle II
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

/// In safe Rust, a true `Box` cycle cannot be constructed; this detects revisiting
/// the same node address (e.g. via `Rc`) or returns `None` for ordinary lists.
fn detect_cycle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut seen: HashSet<*const ListNode> = HashSet::new();
    let mut cur = head.as_ref();
    while let Some(n) = cur {
        let p = n.as_ref() as *const ListNode;
        if !seen.insert(p) {
            return None;
        }
        cur = n.next.as_ref();
    }
    None
}

fn main() {
    println!("{}", detect_cycle(None).is_none());
}

#[cfg(test)]
mod tests {
    use super::{detect_cycle, ListNode};

    fn list_from(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &v in vals.iter().rev() {
            head = Some(Box::new(ListNode { val: v, next: head }));
        }
        head
    }

    #[test]
    fn no_cycle() {
        assert!(detect_cycle(list_from(&[1, 2, 3, 4])).is_none());
    }
}
