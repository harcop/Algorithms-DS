/// LeetCode #3149 - Find the Minimum Cost Array Permutation
fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut memo = vec![vec![-1i32; n]; 1 << n];
    fn dfs(mask: usize, pre: usize, nums: &[i32], n: usize, memo: &mut [Vec<i32>]) -> i32 {
        if mask == (1 << n) - 1 {
            return (pre as i32 - nums[0]).abs();
        }
        if memo[mask][pre] != -1 {
            return memo[mask][pre];
        }
        let mut res = i32::MAX;
        for cur in 1..n {
            if (mask >> cur) & 1 == 0 {
                res = res.min(
                    (pre as i32 - nums[cur]).abs() + dfs(mask | (1 << cur), cur, nums, n, memo),
                );
            }
        }
        memo[mask][pre] = res;
        res
    }
    fn build(
        mask: usize,
        pre: usize,
        nums: &[i32],
        n: usize,
        memo: &mut [Vec<i32>],
        ans: &mut Vec<i32>,
    ) {
        ans.push(pre as i32);
        if mask == (1 << n) - 1 {
            return;
        }
        let res = dfs(mask, pre, nums, n, memo);
        for cur in 1..n {
            if (mask >> cur) & 1 == 0 {
                let cand =
                    (pre as i32 - nums[cur]).abs() + dfs(mask | (1 << cur), cur, nums, n, memo);
                if cand == res {
                    build(mask | (1 << cur), cur, nums, n, memo, ans);
                    break;
                }
            }
        }
    }
    let mut ans = Vec::new();
    build(1, 0, &nums, n, &mut memo, &mut ans);
    ans
}

fn main() {
    println!("{:?}", find_permutation(vec![1, 0, 2]));
}

#[cfg(test)]
mod tests {
    use super::find_permutation;

    #[test]
    fn example1() {
        assert_eq!(find_permutation(vec![1, 0, 2]), vec![0, 1, 2]);
    }

    #[test]
    fn example2() {
        assert_eq!(find_permutation(vec![0, 2, 1]), vec![0, 2, 1]);
    }
}
