/// LeetCode #2003 - Smallest Missing Genetic Value in Each Subtree
fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut g = vec![Vec::new(); n];
    let mut idx = None;
    for (i, &p) in parents.iter().enumerate() {
        if i > 0 {
            g[p as usize].push(i);
        }
        if nums[i] == 1 {
            idx = Some(i);
        }
    }

    let mut ans = vec![1i32; n];
    let Some(mut idx) = idx else {
        return ans;
    };

    let mut vis = vec![false; n];
    let mut has = vec![false; n + 2];
    let mut i = 2usize;

    while idx != usize::MAX {
        fn dfs(
            u: usize,
            g: &[Vec<usize>],
            nums: &[i32],
            vis: &mut [bool],
            has: &mut [bool],
        ) {
            if vis[u] {
                return;
            }
            vis[u] = true;
            let v = nums[u] as usize;
            if v < has.len() {
                has[v] = true;
            }
            for &v in &g[u] {
                dfs(v, g, nums, vis, has);
            }
        }

        dfs(idx, &g, &nums, &mut vis, &mut has);
        while i < has.len() && has[i] {
            i += 1;
        }
        ans[idx] = i as i32;
        idx = if parents[idx] == -1 {
            usize::MAX
        } else {
            parents[idx] as usize
        };
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        smallest_missing_value_subtree(vec![-1, 0, 0, 2], vec![1, 2, 3, 4])
    );
}

#[cfg(test)]
mod tests {
    use super::smallest_missing_value_subtree;

    #[test]
    fn example_one() {
        assert_eq!(
            smallest_missing_value_subtree(vec![-1, 0, 0, 2], vec![1, 2, 3, 4]),
            vec![5, 1, 1, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            smallest_missing_value_subtree(vec![-1, 0, 1, 0, 3, 3], vec![5, 4, 6, 2, 1, 3]),
            vec![7, 1, 1, 4, 2, 1]
        );
    }
}
