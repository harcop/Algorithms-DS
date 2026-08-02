/// LeetCode #2920 - Maximum Points After Collecting Coins From All Nodes
fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    let n = coins.len();
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }

    let mut memo = HashMap::new();
    fn dfs(
        i: usize,
        fa: i32,
        j: usize,
        g: &[Vec<usize>],
        coins: &[i32],
        k: i32,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(&v) = memo.get(&(i, j)) {
            return v;
        }
        let mut a = (coins[i] >> j) - k;
        let mut b = coins[i] >> (j + 1);
        for &c in &g[i] {
            if c as i32 != fa {
                a += dfs(c, i as i32, j, g, coins, k, memo);
                if j < 14 {
                    b += dfs(c, i as i32, j + 1, g, coins, k, memo);
                }
            }
        }
        let ans = a.max(b);
        memo.insert((i, j), ans);
        ans
    }

    dfs(0, -1, 0, &g, &coins, k, &mut memo)
}

fn main() {
    println!(
        "{}",
        maximum_points(
            vec![vec![0, 1], vec![1, 2], vec![2, 3]],
            vec![10, 10, 3, 3],
            5
        )
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_points;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_points(
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                vec![10, 10, 3, 3],
                5
            ),
            11
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_points(vec![vec![0, 1], vec![0, 2]], vec![8, 4, 4], 0),
            16
        );
    }
}
