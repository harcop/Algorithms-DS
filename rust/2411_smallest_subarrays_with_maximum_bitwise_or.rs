/// LeetCode #2411 - Smallest Subarrays With Maximum Bitwise OR
fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut last = vec![-1; 31];
    let mut ans = vec![1; n];

    for i in (0..n).rev() {
        let mut farthest = i as i32;
        for bit in 0..31 {
            if (nums[i] >> bit) & 1 == 1 {
                last[bit] = i as i32;
            }
            if last[bit] != -1 {
                farthest = farthest.max(last[bit]);
            }
        }
        ans[i] = farthest - i as i32 + 1;
    }

    ans
}

fn main() {
    println!("{:?}", smallest_subarrays(vec![1, 0, 2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::smallest_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(smallest_subarrays(vec![1, 0, 2, 1, 3]), vec![3, 3, 2, 2, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_subarrays(vec![1, 2]), vec![2, 1]);
    }
}
