/// LeetCode #259 - 3Sum Smaller (premium)
fn three_sum_smaller(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let n = nums.len();
    let mut count = 0i32;
    for i in 0..n {
        let mut lo = i + 1;
        let mut hi = n - 1;
        while lo < hi {
            let s = nums[i] + nums[lo] + nums[hi];
            if s < target {
                count += (hi - lo) as i32;
                lo += 1;
            } else {
                hi -= 1;
            }
        }
    }
    count
}

fn main() {
    println!("{}", three_sum_smaller(vec![-2, 0, 1, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::three_sum_smaller;

    #[test]
    fn example_one() {
        assert_eq!(three_sum_smaller(vec![-2, 0, 1, 3], 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(three_sum_smaller(vec![], 0), 0);
    }
}
