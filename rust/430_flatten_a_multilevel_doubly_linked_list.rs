/// LeetCode #430 - Flatten Multilevel Doubly Linked List (preorder walk; `prev` unused)
#[derive(Debug, PartialEq)]
pub struct Node {
    pub val: i32,
    pub prev: Option<Box<Node>>,
    pub next: Option<Box<Node>>,
    pub child: Option<Box<Node>>,
}

fn flatten(head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut seq = Vec::new();
    let mut stack = vec![head];

    while let Some(mut top) = stack.pop() {
        let mut cur = top.take();
        while let Some(mut n) = cur {
            seq.push(n.val);
            let nx = n.next.take();
            let ch = n.child.take();
            if let Some(nxt) = nx {
                stack.push(Some(nxt));
            }
            cur = ch;
        }
    }

    let mut list: Option<Box<Node>> = None;
    for v in seq.into_iter().rev() {
        list = Some(Box::new(Node {
            val: v,
            prev: None,
            next: list,
            child: None,
        }));
    }
    list
}

fn to_vec(mut n: Option<Box<Node>>) -> Vec<i32> {
    let mut v = vec![];
    while let Some(b) = n {
        v.push(b.val);
        n = b.next;
    }
    v
}

fn main() {
    let n2 = Some(Box::new(Node {
        val: 2,
        prev: None,
        next: None,
        child: None,
    }));
    let n1 = Some(Box::new(Node {
        val: 1,
        prev: None,
        next: None,
        child: n2,
    }));
    println!("{:?}", flatten(n1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_order() {
        let c = Some(Box::new(Node {
            val: 3,
            prev: None,
            next: None,
            child: None,
        }));
        let mut b = Box::new(Node {
            val: 2,
            prev: None,
            next: None,
            child: c,
        });
        let tail = Box::new(Node {
            val: 4,
            prev: None,
            next: None,
            child: None,
        });
        b.next = Some(tail);

        let head = Some(Box::new(Node {
            val: 1,
            prev: None,
            next: None,
            child: Some(b),
        }));

        assert_eq!(to_vec(flatten(head)), vec![1, 2, 3, 4]);
    }
}
