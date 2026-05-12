/// LeetCode #708 - Insert into a Sorted Circular Linked List
///
/// The list is represented here as a `Vec<i32>` whose elements appear in the
/// order you would encounter them when starting from `head` and traversing the
/// circular list. The function returns a new vector with `val` inserted while
/// maintaining the (cyclic) non-decreasing order.
fn insert_sorted_circular(head: Vec<i32>, val: i32) -> Vec<i32> {
    if head.is_empty() {
        return vec![val];
    }
    let n = head.len();
    let mut out: Vec<i32> = Vec::with_capacity(n + 1);
    let mut inserted = false;
    for i in 0..n {
        let cur = head[i];
        let next = head[(i + 1) % n];
        out.push(cur);
        if !inserted {
            let between = cur <= val && val <= next;
            let wrap = cur > next && (val >= cur || val <= next);
            if between || wrap {
                out.push(val);
                inserted = true;
            }
        }
    }
    if !inserted {
        out.push(val);
    }
    out
}

fn main() {
    println!("{:?}", insert_sorted_circular(vec![3, 4, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::insert_sorted_circular;

    #[test]
    fn empty_list() {
        assert_eq!(insert_sorted_circular(vec![], 5), vec![5]);
    }

    #[test]
    fn between_nodes() {
        assert_eq!(insert_sorted_circular(vec![3, 4, 1], 2), vec![3, 4, 1, 2]);
    }

    #[test]
    fn at_wrap_max() {
        assert_eq!(insert_sorted_circular(vec![3, 4, 1], 5), vec![3, 4, 5, 1]);
    }

    #[test]
    fn at_wrap_min() {
        assert_eq!(insert_sorted_circular(vec![3, 4, 1], 0), vec![3, 4, 0, 1]);
    }

    #[test]
    fn single_element() {
        assert_eq!(insert_sorted_circular(vec![1], 0), vec![1, 0]);
    }

    #[test]
    fn equal_elements() {
        assert_eq!(insert_sorted_circular(vec![1, 1], 1), vec![1, 1, 1]);
    }
}
