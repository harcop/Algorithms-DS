/// LeetCode #148 - Sort List
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vals = Vec::new();
    let mut cur = head;
    while let Some(n) = cur {
        vals.push(n.val);
        cur = n.next;
    }
    vals.sort_unstable();
    let mut h = None;
    for &v in vals.iter().rev() {
        h = Some(Box::new(ListNode { val: v, next: h }));
    }
    h
}

fn main() {
    println!("{:?}", sort_list(None));
}

#[cfg(test)]
mod tests {
    use super::{sort_list, ListNode};

    fn list_from(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut h = None;
        for &v in vals.iter().rev() {
            h = Some(Box::new(ListNode { val: v, next: h }));
        }
        h
    }

    fn to_vec(mut h: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = vec![];
        while let Some(n) = h {
            v.push(n.val);
            h = n.next;
        }
        v
    }

    #[test]
    fn example_one() {
        assert_eq!(to_vec(sort_list(list_from(&[4, 2, 1, 3]))), vec![1, 2, 3, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(to_vec(sort_list(list_from(&[-1, 5, 3, 4, 0]))), vec![-1, 0, 3, 4, 5]);
    }
}
