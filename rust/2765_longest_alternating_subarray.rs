/// LeetCode #2765 - Longest Alternating Subarray
fn alternating_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = -1;
    for i in 0..n {
        let mut k = 1i32;
        let mut j = i;
        while j + 1 < n && nums[j + 1] - nums[j] == k {
            j += 1;
            k *= -1;
        }
        if j - i + 1 > 1 {
            ans = ans.max((j - i + 1) as i32);
        }
    }
    ans
}

fn main() {
    println!("{}", alternating_subarray(vec![2, 3, 4, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::alternating_subarray;

    #[test]
    fn example_one() {
        assert_eq!(alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(alternating_subarray(vec![4, 5, 6]), 2);
    }

    #[test]
    fn example_no_alternating() {
        assert_eq!(alternating_subarray(vec![1, 1, 1]), -1);
    }
}
