/// LeetCode #237 - Delete Node in a Linked List (not tail)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn delete_node(node: &mut ListNode) {
    let mut nxt = node.next.take().unwrap();
    node.val = nxt.val;
    node.next = nxt.next.take();
}

fn main() {
    println!("delete_node demo");
}

#[cfg(test)]
mod tests {
    use super::{delete_node, ListNode};

    fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = vec![];
        let mut c = head.as_ref();
        while let Some(n) = c {
            v.push(n.val);
            c = n.next.as_ref();
        }
        v
    }

    #[test]
    fn example() {
        let mut head = Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: None,
                    })),
                })),
            })),
        });
        delete_node(head.next.as_mut().unwrap());
        assert_eq!(to_vec(&Some(head)), vec![4, 1, 9]);
    }
}
