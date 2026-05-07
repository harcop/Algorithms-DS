/// LeetCode #369 - Plus One Linked List (MSB-first); convert to digits, carry, rebuild
#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn plus_one(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut digs = vec![];
    let mut cur = head;
    while let Some(n) = cur {
        digs.push(n.val);
        cur = n.next;
    }
    let mut carry = 1;
    for x in digs.iter_mut().rev() {
        let s = *x + carry;
        *x = s % 10;
        carry = s / 10;
    }
    if carry > 0 {
        digs.insert(0, carry);
    }
    let mut tail: Option<Box<ListNode>> = None;
    for &x in digs.iter().rev() {
        tail = Some(Box::new(ListNode { val: x, next: tail }));
    }
    tail
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk(v: &[i32]) -> Option<Box<ListNode>> {
        let mut h = None;
        for &x in v.iter().rev() {
            h = Some(Box::new(ListNode { val: x, next: h }));
        }
        h
    }

    fn to_vec(mut h: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = vec![];
        while let Some(n) = h {
            out.push(n.val);
            h = n.next;
        }
        out
    }

    #[test]
    fn ex() {
        assert_eq!(to_vec(plus_one(mk(&[1, 2, 3]))), vec![1, 2, 4]);
        assert_eq!(to_vec(plus_one(mk(&[9, 9]))), vec![1, 0, 0]);
    }
}
