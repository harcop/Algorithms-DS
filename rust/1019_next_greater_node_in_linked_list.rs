/// LeetCode #1019 - Next Greater Node In Linked List
#[derive(Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vals = Vec::new();
    let mut cur = head;
    while let Some(n) = cur {
        vals.push(n.val);
        cur = n.next;
    }
    let mut stack: Vec<usize> = Vec::new();
    let mut ans = vec![0i32; vals.len()];
    for i in 0..vals.len() {
        while let Some(&top) = stack.last() {
            if vals[top] < vals[i] {
                ans[stack.pop().unwrap()] = vals[i];
            } else {
                break;
            }
        }
        stack.push(i);
    }
    ans
}

fn main() {
    let head = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 5, next: None })),
        })),
    }));
    println!("{:?}", next_larger_nodes(head));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let head = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        }));
        assert_eq!(next_larger_nodes(head), vec![5, 5, 0]);
    }
}
