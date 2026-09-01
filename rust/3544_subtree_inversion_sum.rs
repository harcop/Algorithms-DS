/// LeetCode #3544 - Subtree Inversion Sum
fn subtree_inversion_sum(edges: Vec<Vec<i32>>, nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let k = k as usize;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        g[e[0] as usize].push(e[1] as usize);
        g[e[1] as usize].push(e[0] as usize);
    }
    let mut memo = vec![vec![vec![None; 2]; k + 1]; n];
    fn dfs(
        u: usize,
        fa: usize,
        cd: usize,
        parity: usize,
        k: usize,
        g: &[Vec<usize>],
        nums: &[i32],
        memo: &mut [Vec<Vec<Option<i64>>>],
    ) -> i64 {
        if let Some(v) = memo[u][cd][parity] {
            return v;
        }
        let sign = if parity == 0 { 1 } else { -1 };
        let mut res = sign * nums[u] as i64;
        for &v in &g[u] {
            if v != fa {
                res += dfs(v, u, cd.saturating_sub(1), parity, k, g, nums, memo);
            }
        }
        if cd == 0 {
            let mut s = -sign * nums[u] as i64;
            for &v in &g[u] {
                if v != fa {
                    s += dfs(v, u, k - 1, 1 - parity, k, g, nums, memo);
                }
            }
            res = res.max(s);
        }
        memo[u][cd][parity] = Some(res);
        res
    }
    dfs(0, usize::MAX, 0, 0, k, &g, &nums, &mut memo)
}

fn main() {
    println!(
        "{}",
        subtree_inversion_sum(
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]],
            vec![4, -8, -6, 3, 7, -2, 5],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::subtree_inversion_sum;

    #[test]
    fn example1() {
        assert_eq!(
            subtree_inversion_sum(
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]],
                vec![4, -8, -6, 3, 7, -2, 5],
                2
            ),
            27
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            subtree_inversion_sum(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![-1, 3, -2, 4, -5], 2),
            9
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            subtree_inversion_sum(vec![vec![0, 1], vec![0, 2]], vec![0, -1, -2], 3),
            3
        );
    }
}
