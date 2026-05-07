/// LeetCode #435 - Non-overlapping Intervals
fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    let n = intervals.len() as i32;
    if n <= 1 {
        return 0;
    }
    intervals.sort_by_key(|v| v[1]);
    let mut end = intervals[0][1];
    let mut keep = 1;
    for v in intervals.into_iter().skip(1) {
        let (s, e) = (v[0], v[1]);
        if s >= end {
            keep += 1;
            end = e;
        }
    }
    n - keep
}

fn main() {
    println!(
        "{}",
        erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::erase_overlap_intervals;

    #[test]
    fn example_one() {
        assert_eq!(
            erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
    }
}
