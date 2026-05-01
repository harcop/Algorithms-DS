/// LeetCode #143 - Reorder List
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut vals = Vec::new();
    let mut cur = head.take();
    while let Some(node) = cur {
        vals.push(node.val);
        cur = node.next;
    }
    if vals.len() <= 2 {
        *head = vec_to_list(&vals);
        return;
    }
    let mut out = Vec::new();
    let mut l = 0usize;
    let mut r = vals.len() - 1;
    while l <= r {
        out.push(vals[l]);
        if l != r {
            out.push(vals[r]);
        }
        l += 1;
        r -= 1;
    }
    *head = vec_to_list(&out);
}

fn vec_to_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut h = None;
    for &v in values.iter().rev() {
        h = Some(Box::new(ListNode { val: v, next: h }));
    }
    h
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = Vec::new();
    while let Some(n) = head {
        v.push(n.val);
        head = n.next;
    }
    v
}

fn main() {
    let mut h = vec_to_list(&[1, 2, 3, 4]);
    reorder_list(&mut h);
    println!("{:?}", list_to_vec(h));
}

#[cfg(test)]
mod tests {
    use super::{list_to_vec, reorder_list, vec_to_list};

    #[test]
    fn example_one() {
        let mut h = vec_to_list(&[1, 2, 3, 4]);
        reorder_list(&mut h);
        assert_eq!(list_to_vec(h), vec![1, 4, 2, 3]);
    }

    #[test]
    fn example_two() {
        let mut h = vec_to_list(&[1, 2, 3, 4, 5]);
        reorder_list(&mut h);
        assert_eq!(list_to_vec(h), vec![1, 5, 2, 4, 3]);
    }
}
