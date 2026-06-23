/// LeetCode #2054 - Two Best Non-Overlapping Events
fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
    events.sort_by_key(|e| e[0]);
    let n = events.len();
    let mut suffix = vec![0i32; n];
    suffix[n - 1] = events[n - 1][2];
    for i in (0..n - 1).rev() {
        suffix[i] = suffix[i + 1].max(events[i][2]);
    }

    let mut ans = 0i32;
    for e in &events {
        let end = e[1];
        let mut v = e[2];
        let mut lo = 0usize;
        let mut hi = n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if events[mid][0] <= end {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo < n {
            v += suffix[lo];
        }
        ans = ans.max(v);
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_two_events;

    #[test]
    fn example_one() {
        assert_eq!(
            max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]]),
            5
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            max_two_events(vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]]),
            8
        );
    }
}
