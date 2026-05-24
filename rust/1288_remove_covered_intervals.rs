/// LeetCode #1288 - Remove Covered Intervals
fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut max_end = 0i32;
    let mut count = 0i32;
    for v in intervals {
        if v[1] > max_end {
            count += 1;
            max_end = v[1];
        }
    }
    count
}

fn main() {
    println!(
        "{}",
        remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]])
    );
}

#[cfg(test)]
mod tests {
    use super::remove_covered_intervals;

    #[test]
    fn example_one() {
        assert_eq!(
            remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_covered_intervals(vec![vec![1, 4], vec![2, 3]]), 1);
    }
}
