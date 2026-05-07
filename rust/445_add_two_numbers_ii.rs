/// LeetCode #445 - Add Two Numbers II
#[derive(Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut s1 = vec![];
    let mut s2 = vec![];
    while let Some(n) = l1 {
        s1.push(n.val);
        l1 = n.next;
    }
    while let Some(n) = l2 {
        s2.push(n.val);
        l2 = n.next;
    }
    let mut carry = 0;
    let mut head = None;
    while !s1.is_empty() || !s2.is_empty() || carry > 0 {
        let a = s1.pop().unwrap_or(0);
        let b = s2.pop().unwrap_or(0);
        let sum = a + b + carry;
        carry = sum / 10;
        head = Some(Box::new(ListNode {
            val: sum % 10,
            next: head,
        }));
    }
    head
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{add_two_numbers, ListNode};

    #[test]
    fn example_one() {
        let l1 = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: None,
                    })),
                })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        }));
        let mut sum = add_two_numbers(l1, l2);
        let mut got = vec![];
        while let Some(n) = sum.take() {
            got.push(n.val);
            sum = n.next;
        }
        assert_eq!(got, vec![7, 8, 0, 7]);
    }
}
