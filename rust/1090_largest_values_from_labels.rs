/// LeetCode #1090 - Largest Values From Labels
fn largest_vals_from_labels(values: Vec<i32>, labels: Vec<i32>, num_wanted: i32, use_limit: i32) -> i32 {
    let mut by_label: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    for (&v, &l) in values.iter().zip(labels.iter()) {
        by_label.entry(l).or_default().push(v);
    }
    let mut pool = Vec::new();
    for mut vals in by_label.into_values() {
        vals.sort_unstable_by(|a, b| b.cmp(a));
        for v in vals.into_iter().take(use_limit as usize) {
            pool.push(v);
        }
    }
    pool.sort_unstable_by(|a, b| b.cmp(a));
    pool.into_iter()
        .take(num_wanted as usize)
        .sum()
}

fn main() {
    println!(
        "{}",
        largest_vals_from_labels(
            vec![5, 4, 3, 2, 1],
            vec![1, 1, 2, 2, 3],
            3,
            1
        )
    );
}

#[cfg(test)]
mod tests {
    use super::largest_vals_from_labels;

    #[test]
    fn example_one() {
        assert_eq!(
            largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 1, 2, 2, 3], 3, 1),
            9
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 3, 3, 3, 2], 3, 2),
            12
        );
    }
}
