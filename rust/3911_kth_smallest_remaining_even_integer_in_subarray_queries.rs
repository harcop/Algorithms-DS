/// LeetCode #3911 - K-th Smallest Remaining Even Integer in Subarray Queries
fn kth_smallest_remaining_even(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let evens: Vec<(usize, i64)> = nums
        .iter()
        .enumerate()
        .filter(|(_, &x)| x % 2 == 0)
        .map(|(i, &x)| (i, x as i64))
        .collect();
    queries
        .into_iter()
        .map(|q| {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as i64;
            let lo = evens.partition_point(|&(i, _)| i < l);
            let hi = evens.partition_point(|&(i, _)| i <= r);
            let seg = &evens[lo..hi];
            let mut left = k;
            let mut right = k + seg.len() as i64;
            while left < right {
                let mid = (left + right) / 2;
                let cnt = seg.partition_point(|&(_, v)| v <= 2 * mid) as i64;
                if mid - cnt >= k {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            2 * left
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        kth_smallest_remaining_even(vec![1, 4, 7], vec![vec![0, 2, 1], vec![1, 1, 2], vec![0, 0, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::kth_smallest_remaining_even;

    #[test]
    fn example1() {
        assert_eq!(
            kth_smallest_remaining_even(
                vec![1, 4, 7],
                vec![vec![0, 2, 1], vec![1, 1, 2], vec![0, 0, 3]]
            ),
            vec![2, 6, 6]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            kth_smallest_remaining_even(
                vec![2, 5, 8],
                vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]]
            ),
            vec![6, 2, 12]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            kth_smallest_remaining_even(vec![3, 6], vec![vec![0, 1, 1], vec![1, 1, 3]]),
            vec![2, 8]
        );
    }
}
