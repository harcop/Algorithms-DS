/// LeetCode #2046 - Sort Linked List Already Sorted Using Absolute Values
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn append(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let node = Box::new(ListNode { val, next: None });
    if head.is_none() {
        return Some(node);
    }
    let mut cur = head.as_mut().unwrap();
    while cur.next.is_some() {
        cur = cur.next.as_mut().unwrap();
    }
    cur.next = Some(node);
    head
}

fn sort_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vals = Vec::new();
    let mut cur = head;
    while let Some(node) = cur {
        vals.push(node.val);
        cur = node.next;
    }

    let mut ans = None;
    for &v in &vals {
        if v < 0 {
            ans = Some(Box::new(ListNode { val: v, next: ans }));
        }
    }
    for &v in &vals {
        if v >= 0 {
            ans = append(ans, v);
        }
    }
    ans
}

fn vec_to_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut ans = Vec::new();
    while let Some(n) = head {
        ans.push(n.val);
        head = n.next;
    }
    ans
}

fn main() {
    let head = vec_to_list(&[0, 2, -5, 5, 10, -10]);
    println!("{:?}", list_to_vec(sort_linked_list(head)));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, sort_linked_list, vec_to_list};

    #[test]
    fn example_one() {
        let head = vec_to_list(&[0, 2, -5, 5, 10, -10]);
        assert_eq!(
            list_to_vec(sort_linked_list(head)),
            vec![-10, -5, 0, 2, 5, 10]
        );
    }

    #[test]
    fn example_two() {
        let head = vec_to_list(&[0, 1, 2]);
        assert_eq!(list_to_vec(sort_linked_list(head)), vec![0, 1, 2]);
    }
}
