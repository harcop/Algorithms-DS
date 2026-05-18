/// LeetCode #1031 - Maximum Sum of Two Non-Overlapping Subarrays
fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
    fn best(nums: &[i32], len: i32) -> Vec<i32> {
        let len = len as usize;
        let mut window: i32 = nums[..len].iter().sum();
        let mut best = window;
        let mut prefix_best = vec![best];
        for i in len..nums.len() {
            window += nums[i] - nums[i - len];
            best = best.max(window);
            prefix_best.push(best);
        }
        prefix_best
    }
    let a = best(&nums, first_len);
    let b = best(&nums, second_len);
    let mut ans = 0i32;
    for i in first_len as usize..nums.len() {
        ans = ans.max(a[i - 1] + b[b.len() - 1].max(0)); // placeholder
    }
    let n = nums.len();
    let mut res = 0i32;
    for i in first_len as usize..=n - second_len as usize {
        res = res.max(a[i - 1] + window_sum(&nums[i..i + second_len as usize]));
    }
    for i in second_len as usize..=n - first_len as usize {
        res = res.max(b[i - 1] + window_sum(&nums[i..i + first_len as usize]));
    }
    res
}

fn window_sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn main() {
    println!("{}", max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2));
}

#[cfg(test)]
mod tests {
    use super::max_sum_two_no_overlap;

    #[test]
    fn example_one() {
        assert_eq!(max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2), 20);
    }
}
