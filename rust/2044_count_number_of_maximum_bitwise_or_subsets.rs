/// LeetCode #2044 - Count Number of Maximum Bitwise-OR Subsets
fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let mx = nums.iter().fold(0, |acc, x| acc | x);
    let mut ans = 0i32;

    fn dfs(i: usize, nums: &[i32], t: i32, mx: i32, ans: &mut i32) {
        if i == nums.len() {
            if t == mx {
                *ans += 1;
            }
            return;
        }
        dfs(i + 1, nums, t, mx, ans);
        dfs(i + 1, nums, t | nums[i], mx, ans);
    }

    dfs(0, &nums, 0, mx, &mut ans);
    ans
}

fn main() {
    println!("{}", count_max_or_subsets(vec![3, 1]));
}

#[cfg(test)]
mod tests {
    use super::count_max_or_subsets;

    #[test]
    fn example_one() {
        assert_eq!(count_max_or_subsets(vec![3, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_max_or_subsets(vec![2, 2, 2]), 7);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_max_or_subsets(vec![3, 2, 1, 5]), 6);
    }
}
