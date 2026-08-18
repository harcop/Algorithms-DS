/// LeetCode #3263 - Convert Doubly Linked List to Array I
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn to_array(mut head: Option<Box<Node>>) -> Vec<i32> {
    let mut ans = Vec::new();
    while let Some(node) = head {
        ans.push(node.val);
        head = node.next;
    }
    ans
}

fn from_slice(vals: &[i32]) -> Option<Box<Node>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(Node { val: v, next: head }));
    }
    head
}

fn main() {
    println!("{:?}", to_array(from_slice(&[1, 2, 3, 4, 3, 2, 1])));
}

#[cfg(test)]
mod tests {
    use super::{from_slice, to_array};

    #[test]
    fn example1() {
        assert_eq!(
            to_array(from_slice(&[1, 2, 3, 4, 3, 2, 1])),
            vec![1, 2, 3, 4, 3, 2, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(to_array(from_slice(&[2, 2, 2, 2, 2])), vec![2, 2, 2, 2, 2]);
    }

    #[test]
    fn example3() {
        assert_eq!(
            to_array(from_slice(&[3, 2, 3, 2, 3, 2])),
            vec![3, 2, 3, 2, 3, 2]
        );
    }
}
