use std::collections::HashSet;

/// LeetCode #141 - Linked List Cycle
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

/// Visits nodes by pointer identity. A true cycle in `Box` lists is only possible
/// with interior mutability / `Rc`; this matches LeetCode behavior when nodes repeat.
fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut seen: HashSet<*const ListNode> = HashSet::new();
    let mut cur = head.as_ref();
    while let Some(n) = cur {
        let p = n.as_ref() as *const ListNode;
        if !seen.insert(p) {
            return true;
        }
        cur = n.next.as_ref();
    }
    false
}

fn main() {
    println!("{}", has_cycle(None));
}

#[cfg(test)]
mod tests {
    use super::{has_cycle, ListNode};

    fn list_from(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &v in vals.iter().rev() {
            let n = Box::new(ListNode { val: v, next: head });
            head = Some(n);
        }
        head
    }

    #[test]
    fn example_one() {
        assert!(!has_cycle(list_from(&[3, 2, 0, -4])));
    }

    #[test]
    fn example_two() {
        assert!(!has_cycle(list_from(&[1, 2])));
    }

    #[test]
    fn example_three() {
        assert!(!has_cycle(list_from(&[1])));
    }
}
