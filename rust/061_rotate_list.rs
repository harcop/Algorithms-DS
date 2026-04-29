/// LeetCode #61 - Rotate List
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut vals = list_to_vec(head);
    if vals.is_empty() {
        return None;
    }
    let k = (k as usize) % vals.len();
    vals.rotate_right(k);
    vec_to_list(&vals)
}

fn vec_to_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in values.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
}
fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![];
    while let Some(node) = head {
        out.push(node.val);
        head = node.next;
    }
    out
}

fn main() {
    println!("{:?}", list_to_vec(rotate_right(vec_to_list(&[1,2,3,4,5]), 2)));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, rotate_right, vec_to_list};
    #[test]
    fn example_one() {
        assert_eq!(list_to_vec(rotate_right(vec_to_list(&[1,2,3,4,5]), 2)), vec![4,5,1,2,3]);
    }
    #[test]
    fn example_two() {
        assert_eq!(list_to_vec(rotate_right(vec_to_list(&[0,1,2]), 4)), vec![2,0,1]);
    }
}
