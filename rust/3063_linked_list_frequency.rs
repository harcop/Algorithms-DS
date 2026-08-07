/// LeetCode #3063 - Linked List Frequency
use std::collections::HashMap;

fn linked_list_frequency(head: Vec<i32>) -> Vec<i32> {
    let mut freq = HashMap::new();
    for x in head {
        *freq.entry(x).or_insert(0) += 1;
    }
    freq.into_values().collect()
}

fn main() {
    let mut v = linked_list_frequency(vec![1, 1, 2, 1, 2, 3]);
    v.sort_unstable();
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::linked_list_frequency;

    fn assert_multiset_eq(mut got: Vec<i32>, mut want: Vec<i32>) {
        got.sort_unstable();
        want.sort_unstable();
        assert_eq!(got, want);
    }

    #[test]
    fn example1() {
        assert_multiset_eq(linked_list_frequency(vec![1, 1, 2, 1, 2, 3]), vec![3, 2, 1]);
    }

    #[test]
    fn example2() {
        assert_multiset_eq(linked_list_frequency(vec![1, 1, 2, 2, 2]), vec![2, 3]);
    }

    #[test]
    fn example3() {
        assert_multiset_eq(
            linked_list_frequency(vec![6, 5, 4, 3, 2, 1]),
            vec![1, 1, 1, 1, 1, 1],
        );
    }
}
