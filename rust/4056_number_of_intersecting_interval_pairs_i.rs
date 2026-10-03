/// LeetCode #4056 - Number of Intersecting Interval Pairs I
fn count_intersecting_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    count_pairs(&intervals) as i32
}

fn count_pairs(intervals: &[Vec<i32>]) -> i64 {
    let n = intervals.len();
    let mut starts = Vec::with_capacity(n);
    let mut ends = Vec::with_capacity(n);
    for iv in intervals {
        starts.push(iv[0]);
        ends.push(iv[1]);
    }
    starts.sort_unstable();
    ends.sort_unstable();
    let mut ans = n as i64 * (n as i64 - 1) / 2;
    let mut i = 0;
    for start in starts {
        while i < n && ends[i] < start {
            i += 1;
        }
        ans -= i as i64;
    }
    ans
}

fn main() {
    println!(
        "{}",
        count_intersecting_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_intersecting_intervals;

    #[test]
    fn example1() {
        assert_eq!(
            count_intersecting_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_intersecting_intervals(vec![vec![1, 5], vec![2, 4], vec![3, 6]]),
            3
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_intersecting_intervals(vec![vec![1, 2], vec![3, 4], vec![5, 6]]),
            0
        );
    }
}
