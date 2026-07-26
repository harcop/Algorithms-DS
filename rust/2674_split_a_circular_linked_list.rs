/// LeetCode #2674 - Split a Circular Linked List
/// Represented as a circular Vec for testing; split returns two circular halves as Vecs.
fn split_circular_linked_list(list: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let n = list.len();
    let mid = (n + 1) / 2; // ceil(n / 2)
    let first = list[..mid].to_vec();
    let second = list[mid..].to_vec();
    (first, second)
}

fn main() {
    println!("{:?}", split_circular_linked_list(vec![1, 5, 7]));
}

#[cfg(test)]
mod tests {
    use super::split_circular_linked_list;

    #[test]
    fn example_one() {
        assert_eq!(
            split_circular_linked_list(vec![1, 5, 7]),
            (vec![1, 5], vec![7])
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            split_circular_linked_list(vec![2, 6, 1, 5]),
            (vec![2, 6], vec![1, 5])
        );
    }
}
