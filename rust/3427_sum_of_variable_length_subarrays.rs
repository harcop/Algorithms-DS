/// LeetCode #3427 - Sum of Variable Length Subarrays
fn subarray_sum(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut pref = vec![0i32; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + nums[i];
    }
    let mut ans = 0;
    for (i, &x) in nums.iter().enumerate() {
        let start = i.saturating_sub(x as usize);
        ans += pref[i + 1] - pref[start];
    }
    ans
}

fn main() {
    println!("{}", subarray_sum(vec![2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::subarray_sum;

    #[test]
    fn example1() {
        assert_eq!(subarray_sum(vec![2, 3, 1]), 11);
    }

    #[test]
    fn example2() {
        assert_eq!(subarray_sum(vec![3, 1, 1, 2]), 13);
    }
}
