/// LeetCode #2968 - Apply Operations to Maximize Frequency Score
fn max_frequency_score(mut nums: Vec<i32>, k: i64) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut s = vec![0i64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + nums[i] as i64;
    }
    let mut lo = 0usize;
    let mut hi = n;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        let mut ok = false;
        for i in 0..=n - mid {
            let j = i + mid;
            let m = (i + j) / 2;
            let left = (m - i) as i64 * nums[m] as i64 - (s[m] - s[i]);
            let right = (s[j] - s[m]) - (j - m) as i64 * nums[m] as i64;
            if left + right <= k {
                ok = true;
                break;
            }
        }
        if ok {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", max_frequency_score(vec![1, 2, 6, 4], 3));
}

#[cfg(test)]
mod tests {
    use super::max_frequency_score;

    #[test]
    fn example_one() {
        assert_eq!(max_frequency_score(vec![1, 2, 6, 4], 3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_frequency_score(vec![1, 4, 4, 2, 4], 0), 3);
    }
}
