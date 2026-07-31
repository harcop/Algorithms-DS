/// LeetCode #2824 - Count Pairs Whose Sum is Less Than Target
fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut ans = 0i32;
    let mut l = 0usize;
    let mut r = nums.len();
    while l < r {
        if nums[l] + nums[r - 1] < target {
            ans += (r - l - 1) as i32;
            l += 1;
        } else {
            r -= 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_pairs(vec![-1, 1, 2, 3, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::count_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_pairs(vec![-1, 1, 2, 3, 1], 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2), 10);
    }
}
