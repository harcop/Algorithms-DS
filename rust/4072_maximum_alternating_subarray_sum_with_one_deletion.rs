/// LeetCode #4072 - Maximum Alternating Subarray Sum With One Deletion
fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut end_odd = vec![0i64; n];
    let mut end_even = vec![0i64; n];
    end_odd[0] = nums[0] as i64;
    let mut ans = end_odd[0];
    for i in 1..n {
        let x = nums[i] as i64;
        end_odd[i] = x + end_even[i - 1].max(0);
        end_even[i] = end_odd[i - 1] - x;
        ans = ans.max(end_odd[i]).max(end_even[i]);
    }

    let mut weighted = vec![0i64; n + 1];
    for i in 0..n {
        let sign = if i % 2 == 0 { 1 } else { -1 };
        weighted[i + 1] = weighted[i] + sign * nums[i] as i64;
    }
    let mut suf_max = vec![0i64; n + 1];
    let mut suf_min = vec![0i64; n + 1];
    suf_max[n] = weighted[n];
    suf_min[n] = weighted[n];
    for i in (0..n).rev() {
        suf_max[i] = weighted[i].max(suf_max[i + 1]);
        suf_min[i] = weighted[i].min(suf_min[i + 1]);
    }

    for d in 1..n.saturating_sub(1) {
        let left = d - 1;
        let right = d + 1;
        let hi = suf_max[right + 1];
        let lo = suf_min[right + 1];
        let (start_max, start_min) = if right % 2 == 0 {
            (hi - weighted[right], lo - weighted[right])
        } else {
            (weighted[right] - lo, weighted[right] - hi)
        };
        if left >= 1 {
            ans = ans.max(end_even[left] + start_max);
        }
        ans = ans.max(end_odd[left] - start_min);
    }
    ans
}

fn main() {
    println!("{}", max_alternating_sum(vec![5, -5, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_alternating_sum;

    fn brute(nums: &[i32]) -> i64 {
        let n = nums.len();
        let mut best = i64::MIN;
        let score = |a: &[i32]| -> i64 {
            a.iter().enumerate().fold(0i64, |acc, (i, &x)| {
                if i % 2 == 0 { acc + x as i64 } else { acc - x as i64 }
            })
        };
        for del in 0..=n {
            let mut kept = Vec::with_capacity(n);
            for (i, &x) in nums.iter().enumerate() {
                if i + 1 != del {
                    kept.push(x);
                }
            }
            if kept.is_empty() {
                continue;
            }
            for l in 0..kept.len() {
                for r in l..kept.len() {
                    best = best.max(score(&kept[l..=r]));
                }
            }
        }
        best
    }

    #[test]
    fn example1() {
        assert_eq!(max_alternating_sum(vec![5, -5, 1]), 11);
    }

    #[test]
    fn example2() {
        assert_eq!(max_alternating_sum(vec![10, -5, -100]), 110);
    }

    #[test]
    fn example3() {
        assert_eq!(max_alternating_sum(vec![4, 7]), 7);
    }

    #[test]
    fn matches_brute_on_small_arrays() {
        let samples = [
            vec![1],
            vec![-3],
            vec![3, 1, -4, 10],
            vec![1, -2, 3, -4, 5],
            vec![-8, 2, -3, 9, -1],
            vec![4, -1, 4, -1],
        ];
        for nums in samples {
            assert_eq!(max_alternating_sum(nums.clone()), brute(&nums));
        }
    }
}
