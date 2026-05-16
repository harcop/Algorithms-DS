/// LeetCode #851 - Loud and Rich
fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    let n = quiet.len();
    let mut g = vec![vec![]; n];
    for e in richer {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[b].push(a);
    }
    let mut memo = vec![-1i32; n];

    fn dfs(u: usize, g: &[Vec<usize>], quiet: &[i32], memo: &mut [i32]) -> i32 {
        if memo[u] != -1 {
            return memo[u];
        }
        memo[u] = u as i32;
        for &v in &g[u] {
            let cand = dfs(v, g, quiet, memo);
            if quiet[cand as usize] < quiet[memo[u] as usize] {
                memo[u] = cand;
            }
        }
        memo[u]
    }

    (0..n).map(|i| dfs(i, &g, &quiet, &mut memo)).collect()
}

fn main() {
    println!(
        "{:?}",
        loud_and_rich(vec![vec![1, 0], vec![2, 1]], vec![1, 0, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::loud_and_rich;

    #[test]
    fn example_one() {
        assert_eq!(
            loud_and_rich(vec![vec![1, 0], vec![2, 1]], vec![1, 0, 2]),
            vec![1, 0, 1]
        );
    }
}
