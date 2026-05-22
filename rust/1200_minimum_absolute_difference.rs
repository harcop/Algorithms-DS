/// LeetCode #1200 - Minimum Absolute Difference
fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    arr.sort_unstable();
    let mut diff = i32::MAX;
    for w in arr.windows(2) {
        diff = diff.min(w[1] - w[0]);
    }
    arr.windows(2)
        .filter(|w| w[1] - w[0] == diff)
        .map(|w| vec![w[0], w[1]])
        .collect()
}

fn main() {
    println!("{:?}", minimum_abs_difference(vec![4, 2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::minimum_abs_difference;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_abs_difference(vec![4, 2, 1, 3]),
            vec![vec![1, 2], vec![2, 3], vec![3, 4]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_abs_difference(vec![1, 3, 6, 10, 15]), vec![vec![1, 3]]);
    }
}
