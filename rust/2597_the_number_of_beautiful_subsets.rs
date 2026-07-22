/// LeetCode #2597 - The Number of Beautiful Subsets
fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = -1;
    let mut cnt = [0i32; 1010];
    let n = nums.len();

    fn dfs(i: usize, nums: &[i32], k: i32, cnt: &mut [i32; 1010], ans: &mut i32, n: usize) {
        if i >= n {
            *ans += 1;
            return;
        }
        dfs(i + 1, nums, k, cnt, ans, n);
        let x = nums[i] as usize;
        let ok1 = x + k as usize >= cnt.len() || cnt[x + k as usize] == 0;
        let ok2 = nums[i] - k < 0 || cnt[(nums[i] - k) as usize] == 0;
        if ok1 && ok2 {
            cnt[x] += 1;
            dfs(i + 1, nums, k, cnt, ans, n);
            cnt[x] -= 1;
        }
    }

    dfs(0, &nums, k, &mut cnt, &mut ans, n);
    ans
}

fn main() {
    println!("{}", beautiful_subsets(vec![2, 4, 6], 2));
}

#[cfg(test)]
mod tests {
    use super::beautiful_subsets;

    #[test]
    fn example_one() {
        assert_eq!(beautiful_subsets(vec![2, 4, 6], 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(beautiful_subsets(vec![1], 1), 1);
    }
}
