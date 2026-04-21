/// LeetCode #16 - 3Sum Closest
fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut closest = nums[0] + nums[1] + nums[2];

    for i in 0..(n - 2) {
        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if (sum - target).abs() < (closest - target).abs() {
                closest = sum;
            }

            if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            } else {
                return target;
            }
        }
    }

    closest
}

fn main() {
    println!("{}", three_sum_closest(vec![-1, 2, 1, -4], 1));
}

#[cfg(test)]
mod tests {
    use super::three_sum_closest;

    #[test]
    fn example_one() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}
