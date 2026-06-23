/// LeetCode #2058 - Find the Minimum and Maximum Number of Nodes Between Critical Points
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut min_dist = i32::MAX;
    let mut max_dist = -1i32;
    let mut first = -1i32;
    let mut last = -1i32;
    let mut i = 0i32;
    let mut cur = head;

    while let Some(node) = cur {
        if let Some(next) = &node.next {
            if let Some(after) = &next.next {
                let a = node.val;
                let b = next.val;
                let c = after.val;
                if (a > b && b < c) || (a < b && b > c) {
                    if last == -1 {
                        first = i;
                        last = i;
                    } else {
                        min_dist = min_dist.min(i - last);
                        last = i;
                        max_dist = max_dist.max(last - first);
                    }
                }
            } else {
                break;
            }
        } else {
            break;
        }
        i += 1;
        cur = node.next;
    }

    if first == last {
        vec![-1, -1]
    } else {
        vec![min_dist, max_dist]
    }
}

fn vec_to_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

fn main() {
    println!("{:?}", nodes_between_critical_points(vec_to_list(&[3, 1])));
}

#[cfg(test)]
mod tests {
    use super::{nodes_between_critical_points, vec_to_list};

    #[test]
    fn example_one() {
        assert_eq!(
            nodes_between_critical_points(vec_to_list(&[3, 1])),
            vec![-1, -1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            nodes_between_critical_points(vec_to_list(&[5, 3, 1, 2, 5, 1, 2])),
            vec![1, 3]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            nodes_between_critical_points(vec_to_list(&[1, 3, 2, 2, 3, 2, 2, 2, 7])),
            vec![3, 3]
        );
    }
}
