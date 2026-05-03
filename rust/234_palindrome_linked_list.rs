/// LeetCode #234 - Palindrome Linked List
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut vals = vec![];
    let mut c = head.as_ref();
    while let Some(n) = c {
        vals.push(n.val);
        c = n.next.as_ref();
    }
    let mut i = 0usize;
    let mut j = vals.len();
    while i < j {
        j -= 1;
        if vals[i] != vals[j] {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    println!("{}", is_palindrome(None));
}

#[cfg(test)]
mod tests {
    use super::{is_palindrome, ListNode};

    fn list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut h = None;
        for &v in vals.iter().rev() {
            h = Some(Box::new(ListNode { val: v, next: h }));
        }
        h
    }

    #[test]
    fn example_one() {
        assert!(is_palindrome(list(&[1, 2, 2, 1])));
    }

    #[test]
    fn example_two() {
        assert!(!is_palindrome(list(&[1, 2])));
    }
}
