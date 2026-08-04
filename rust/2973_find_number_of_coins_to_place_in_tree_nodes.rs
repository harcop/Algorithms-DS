/// LeetCode #2973 - Find Number of Coins to Place in Tree Nodes
fn placed_coins(edges: Vec<Vec<i32>>, cost: Vec<i32>) -> Vec<i64> {
    let n = cost.len();
    let mut g = vec![vec![]; n];
    for e in edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }
    let mut ans = vec![1i64; n];
    fn dfs(a: usize, fa: i32, g: &[Vec<usize>], cost: &[i32], ans: &mut [i64]) -> Vec<i64> {
        let mut res = vec![cost[a] as i64];
        for &b in &g[a] {
            if b as i32 != fa {
                res.extend(dfs(b, a as i32, g, cost, ans));
            }
        }
        res.sort_unstable();
        if res.len() >= 3 {
            let m = res.len();
            let p1 = res[m - 3] * res[m - 2] * res[m - 1];
            let p2 = res[0] * res[1] * res[m - 1];
            ans[a] = p1.max(p2).max(0);
        }
        if res.len() > 5 {
            let mut trimmed = res[..2].to_vec();
            trimmed.extend_from_slice(&res[res.len() - 3..]);
            return trimmed;
        }
        res
    }
    dfs(0, -1, &g, &cost, &mut ans);
    ans
}

fn main() {
    println!("{:?}", placed_coins(vec![vec![0, 1], vec![0, 2]], vec![1, 2, -2]));
}

#[cfg(test)]
mod tests {
    use super::placed_coins;

    #[test]
    fn example_one() {
        assert_eq!(
            placed_coins(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]],
                vec![1, 2, 3, 4, 5, 6]
            ),
            vec![120, 1, 1, 1, 1, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            placed_coins(
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 6],
                    vec![2, 7],
                    vec![2, 8]
                ],
                vec![1, 4, 2, 3, 5, 7, 8, -4, 2]
            ),
            vec![280, 140, 32, 1, 1, 1, 1, 1, 1]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            placed_coins(vec![vec![0, 1], vec![0, 2]], vec![1, 2, -2]),
            vec![0, 1, 1]
        );
    }
}
