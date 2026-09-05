/// LeetCode #3576 - Transform Array to All Equal Elements
fn check(nums: &[i32], target: i32, k: i32) -> bool {
    let mut cnt = 0;
    let mut sign = 1;
    for i in 0..nums.len() - 1 {
        let x = nums[i] * sign;
        if x == target {
            sign = 1;
        } else {
            sign = -1;
            cnt += 1;
        }
    }
    cnt <= k && nums[nums.len() - 1] * sign == target
}

fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
    check(&nums, nums[0], k) || check(&nums, -nums[0], k)
}

fn main() {
    println!("{}", can_make_equal(vec![1, -1, 1, -1, 1], 3));
}

#[cfg(test)]
mod tests {
    use super::can_make_equal;

    #[test]
    fn example1() {
        assert_eq!(can_make_equal(vec![1, -1, 1, -1, 1], 3), true);
    }

    #[test]
    fn example2() {
        assert_eq!(can_make_equal(vec![-1, -1, -1, 1, 1, 1], 5), false);
    }
}
