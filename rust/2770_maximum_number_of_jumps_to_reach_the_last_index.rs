/// LeetCode #2770 - Maximum Number of Jumps to Reach the Last Index
fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut f = vec![i32::MIN; n];
    f[n - 1] = 0;
    fn dfs(i: usize, nums: &[i32], target: i32, f: &mut Vec<i32>) -> i32 {
        if i == nums.len() - 1 {
            return 0;
        }
        if f[i] != i32::MIN {
            return f[i];
        }
        f[i] = -(1 << 30);
        for j in i + 1..nums.len() {
            if (nums[i] - nums[j]).abs() <= target {
                let sub = dfs(j, nums, target, f);
                if sub != -(1 << 30) {
                    f[i] = f[i].max(1 + sub);
                }
            }
        }
        f[i]
    }
    // reset f[n-1] before calling so dfs returns 0 correctly
    f[n - 1] = i32::MIN;
    let ans = dfs(0, &nums, target, &mut f);
    if ans < 0 { -1 } else { ans }
}

fn main() {
    println!("{}", maximum_jumps(vec![1, 3, 6, 4, 1, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::maximum_jumps;

    #[test]
    fn example_one() {
        assert_eq!(maximum_jumps(vec![1, 3, 6, 4, 1, 2], 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_jumps(vec![1, 3, 6, 4, 1, 2], 3), 5);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_jumps(vec![1, 3, 6, 4, 1, 2], 0), -1);
    }
}
