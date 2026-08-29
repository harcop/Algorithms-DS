/// LeetCode #3469 - Find Minimum Cost to Remove Array Elements
fn min_cost(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut mem = vec![vec![-1i32; n + 1]; n + 1];
    fn dp(last: usize, i: usize, nums: &[i32], mem: &mut [Vec<i32>]) -> i32 {
        let n = nums.len();
        if i == n {
            return nums[last];
        }
        if i == n - 1 {
            return nums[last].max(nums[i]);
        }
        if mem[i][last] != -1 {
            return mem[i][last];
        }
        let a = nums[i].max(nums[i + 1]) + dp(last, i + 2, nums, mem);
        let b = nums[last].max(nums[i]) + dp(i + 1, i + 2, nums, mem);
        let c = nums[last].max(nums[i + 1]) + dp(i, i + 2, nums, mem);
        mem[i][last] = a.min(b).min(c);
        mem[i][last]
    }
    dp(0, 1, &nums, &mut mem)
}

fn main() {
    println!("{}", min_cost(vec![6, 2, 8, 4]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cost(vec![6, 2, 8, 4]), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost(vec![2, 1, 3, 3]), 5);
    }
}
