/// LeetCode #2095 - Delete the Middle Node of a Linked List
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vals = Vec::new();
    let mut cur = head;
    while let Some(node) = cur {
        vals.push(node.val);
        cur = node.next;
    }

    if vals.len() <= 1 {
        return None;
    }

    let mid = vals.len() / 2;
    let mut ans = None;
    for (i, &v) in vals.iter().enumerate().rev() {
        if i != mid {
            ans = Some(Box::new(ListNode { val: v, next: ans }));
        }
    }
    ans
}

fn main() {
    let head = list(&[1, 3, 4, 7, 1, 2, 6]);
    println!("{:?}", to_vec(&delete_middle(head)));
}

fn list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut h = None;
    for &v in vals.iter().rev() {
        h = Some(Box::new(ListNode { val: v, next: h }));
    }
    h
}

fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut vals = Vec::new();
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        vals.push(node.val);
        cur = node.next.as_ref();
    }
    vals
}

#[cfg(test)]
mod tests {
    use super::{delete_middle, list, to_vec};

    #[test]
    fn example_one() {
        let out = delete_middle(list(&[1, 3, 4, 7, 1, 2, 6]));
        assert_eq!(to_vec(&out), vec![1, 3, 4, 1, 2, 6]);
    }

    #[test]
    fn example_two() {
        let out = delete_middle(list(&[1, 2, 3, 4]));
        assert_eq!(to_vec(&out), vec![1, 2, 4]);
    }

    #[test]
    fn example_three() {
        assert!(delete_middle(list(&[2, 1])).is_some());
        assert!(delete_middle(list(&[1])).is_none());
    }
}
