/// LeetCode #4067 - Longest Subarray With Restricted Pair Sums
fn max_subarray(nums: Vec<i32>) -> i32 {
    let mx = nums.iter().copied().max().unwrap_or(0) as usize;
    let mut cnt_sum = vec![0i32; (mx << 1) | 1];
    let mut cnt_diff = vec![0i32; mx + 1];
    let mut ans = 0i32;
    let mut l = 0usize;
    for (r, &x) in nums.iter().enumerate() {
        let x = x as usize;
        while cnt_sum[x] > 0 || cnt_diff[x] > 0 {
            let y = nums[l] as usize;
            l += 1;
            for &z in &nums[l..r] {
                let z = z as usize;
                cnt_sum[y + z] -= 1;
                cnt_diff[y.abs_diff(z)] -= 1;
            }
        }
        for &y in &nums[l..r] {
            let y = y as usize;
            cnt_sum[x + y] += 1;
            cnt_diff[x.abs_diff(y)] += 1;
        }
        ans = ans.max((r - l + 1) as i32);
    }
    ans
}

fn main() {
    println!("{}", max_subarray(vec![2, 3, 5, 3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_subarray;

    #[test]
    fn example1() {
        assert_eq!(max_subarray(vec![2, 3, 5, 3, 2, 1]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_subarray(vec![3, 4, 5, 6]), 4);
    }
}
