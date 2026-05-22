/// LeetCode #1207 - Unique Number of Occurrences
fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut cnt = std::collections::HashMap::new();
    for x in arr {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let mut seen = std::collections::HashSet::new();
    for v in cnt.values() {
        if !seen.insert(*v) {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::unique_occurrences;

    #[test]
    fn example_one() {
        assert!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
    }

    #[test]
    fn example_two() {
        assert!(!unique_occurrences(vec![1, 2]));
    }

    #[test]
    fn example_three() {
        assert!(unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]));
    }
}
