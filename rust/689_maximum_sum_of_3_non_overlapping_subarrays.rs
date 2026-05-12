/// LeetCode #689 - Maximum Sum of 3 Non-Overlapping Subarrays
fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut window = vec![0i32; n - k + 1];
    let mut s: i32 = nums[..k].iter().sum();
    window[0] = s;
    for i in k..n {
        s += nums[i] - nums[i - k];
        window[i - k + 1] = s;
    }
    let m = window.len();
    let mut left = vec![0usize; m];
    let mut best = 0usize;
    for i in 0..m {
        if window[i] > window[best] {
            best = i;
        }
        left[i] = best;
    }
    let mut right = vec![0usize; m];
    let mut best = m - 1;
    for i in (0..m).rev() {
        if window[i] >= window[best] {
            best = i;
        }
        right[i] = best;
    }
    let mut ans = vec![0usize; 3];
    let mut max_sum = -1i32;
    for j in k..=m - k - 1 {
        let l = left[j - k];
        let r = right[j + k];
        let total = window[l] + window[j] + window[r];
        if total > max_sum {
            max_sum = total;
            ans = vec![l, j, r];
        }
    }
    ans.iter().map(|&x| x as i32).collect()
}

fn main() {
    println!(
        "{:?}",
        max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::max_sum_of_three_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(
            max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2),
            vec![0, 3, 5]
        );
    }
}
