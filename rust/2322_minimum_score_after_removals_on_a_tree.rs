/// LeetCode #2322 - Minimum Score After Removals on a Tree
fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut g = vec![vec![]; n];
    for e in &edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }

    let s = nums.iter().fold(0, |acc, &x| acc ^ x);
    let mut ans = i32::MAX;

    fn dfs(i: usize, fa: usize, g: &[Vec<usize>], nums: &[i32]) -> i32 {
        let mut res = nums[i];
        for &j in &g[i] {
            if j != fa {
                res ^= dfs(j, i, g, nums);
            }
        }
        res
    }

    fn dfs2(
        i: usize,
        fa: usize,
        g: &[Vec<usize>],
        nums: &[i32],
        s: i32,
        s1: i32,
        ans: &mut i32,
    ) -> i32 {
        let mut res = nums[i];
        for &j in &g[i] {
            if j != fa {
                let s2 = dfs2(j, i, g, nums, s, s1, ans);
                res ^= s2;
                let mx = (s ^ s1).max(s2).max(s1 ^ s2);
                let mn = (s ^ s1).min(s2).min(s1 ^ s2);
                *ans = (*ans).min(mx - mn);
            }
        }
        res
    }

    for i in 0..n {
        for &j in &g[i] {
            let s1 = dfs(i, j, &g, &nums);
            dfs2(i, j, &g, &nums, s, s1, &mut ans);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        minimum_score(vec![1, 5, 5, 4, 11], vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_score;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_score(vec![1, 5, 5, 4, 11], vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]]),
            9
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_score(
                vec![5, 5, 2, 4, 4, 2],
                vec![vec![0, 1], vec![1, 2], vec![5, 2], vec![4, 3], vec![1, 3]]
            ),
            0
        );
    }
}
