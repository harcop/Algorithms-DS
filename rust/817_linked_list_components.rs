/// LeetCode #817 - Linked List Components
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

use std::collections::HashSet;

fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut p = head.as_ref();
    let mut in_component = false;
    let mut ans = 0i32;
    while let Some(n) = p {
        if set.contains(&n.val) {
            if !in_component {
                ans += 1;
                in_component = true;
            }
        } else {
            in_component = false;
        }
        p = n.next.as_ref();
    }
    ans
}

fn main() {
    let mut n3 = Box::new(ListNode { val: 3, next: None });
    let mut n2 = Box::new(ListNode {
        val: 2,
        next: Some(n3),
    });
    let mut n1 = Box::new(ListNode {
        val: 1,
        next: Some(n2),
    });
    println!("{}", num_components(Some(n1), vec![0, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::{num_components, ListNode};

    #[test]
    fn example_one() {
        let mut n3 = Box::new(ListNode { val: 3, next: None });
        let n2 = Box::new(ListNode {
            val: 2,
            next: Some(n3),
        });
        let n1 = Box::new(ListNode {
            val: 1,
            next: Some(n2),
        });
        assert_eq!(num_components(Some(n1), vec![0, 1, 3]), 2);
    }
}
