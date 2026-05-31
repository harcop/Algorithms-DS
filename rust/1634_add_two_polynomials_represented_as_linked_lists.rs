/// LeetCode #1634 - Add Two Polynomials Represented As Linked Lists
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PolyNode {
    pub val: i32,
    pub power: i32,
    pub next: Option<Box<PolyNode>>,
}

fn add_poly_node(poly1: Option<Box<PolyNode>>, poly2: Option<Box<PolyNode>>) -> Option<Box<PolyNode>> {
    let mut map = std::collections::BTreeMap::new();
    let mut add = |mut head: Option<Box<PolyNode>>| {
        while let Some(n) = head {
            *map.entry(n.power).or_insert(0) += n.val;
            head = n.next;
        }
    };
    add(poly1);
    add(poly2);
    let mut dummy = PolyNode { val: 0, power: 0, next: None };
    let mut tail = &mut dummy;
    for (p, v) in map {
        if v != 0 {
            tail.next = Some(Box::new(PolyNode { val: v, power: p, next: None }));
            tail = tail.next.as_mut().unwrap();
        }
    }
    dummy.next
}
fn main() { println!("{:?}", add_poly_node(None, None)); }
#[cfg(test)]
mod tests {
    use super::{add_poly_node, PolyNode};
    fn build(v: Vec<(i32, i32)>) -> Option<Box<PolyNode>> {
        let mut head = None;
        for (val, power) in v.into_iter().rev() {
            head = Some(Box::new(PolyNode { val, power, next: head }));
        }
        head
    }
    #[test]
    fn example_one() {
        let a = build(vec![(1, 1)]);
        let b = build(vec![(1, 0)]);
        let r = add_poly_node(a, b);
        assert!(r.is_some());
    }
}