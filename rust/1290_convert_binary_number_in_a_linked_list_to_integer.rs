/// LeetCode #1290 - Convert Binary Number in a Linked List to Integer
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut ans = 0;
    let mut cur = head.as_ref();
    while let Some(n) = cur {
        ans = ans * 2 + n.val;
        cur = n.next.as_ref();
    }
    ans
}

fn main() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })),
    }));
    println!("{}", get_decimal_value(head));
}

#[cfg(test)]
mod tests {
    use super::{get_decimal_value, ListNode};

    fn list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut h = None;
        for &v in vals.iter().rev() {
            h = Some(Box::new(ListNode { val: v, next: h }));
        }
        h
    }

    #[test]
    fn example_one() {
        assert_eq!(get_decimal_value(list(&[1, 0, 1])), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_decimal_value(list(&[0])), 0);
    }
}
