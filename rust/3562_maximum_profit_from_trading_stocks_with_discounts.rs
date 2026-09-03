/// LeetCode #3562 - Maximum Profit from Trading Stocks with Discounts
fn max_profit(
    n: i32,
    present: Vec<i32>,
    future: Vec<i32>,
    hierarchy: Vec<Vec<i32>>,
    budget: i32,
) -> i32 {
    let n = n as usize;
    let budget = budget as usize;
    let mut g = vec![Vec::new(); n + 1];
    for e in &hierarchy {
        g[e[0] as usize].push(e[1] as usize);
    }
    fn dfs(
        u: usize,
        budget: usize,
        present: &[i32],
        future: &[i32],
        g: &[Vec<usize>],
    ) -> Vec<[i32; 2]> {
        let mut nxt = vec![[0, 0]; budget + 1];
        for &v in &g[u] {
            let fv = dfs(v, budget, present, future, g);
            for j in (0..=budget).rev() {
                for jv in 0..=j {
                    for pre in 0..2 {
                        let val = nxt[j - jv][pre] + fv[jv][pre];
                        if val > nxt[j][pre] {
                            nxt[j][pre] = val;
                        }
                    }
                }
            }
        }
        let mut f = vec![[0, 0]; budget + 1];
        let price = future[u - 1];
        for j in 0..=budget {
            for pre in 0..2 {
                let cost = (present[u - 1] / (pre as i32 + 1)) as usize;
                if j >= cost {
                    f[j][pre] = nxt[j][0].max(nxt[j - cost][1] + (price - cost as i32));
                } else {
                    f[j][pre] = nxt[j][0];
                }
            }
        }
        f
    }
    dfs(1, budget, &present, &future, &g)[budget][0]
}

fn main() {
    println!(
        "{}",
        max_profit(2, vec![1, 2], vec![4, 3], vec![vec![1, 2]], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example1() {
        assert_eq!(max_profit(2, vec![1, 2], vec![4, 3], vec![vec![1, 2]], 3), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(max_profit(2, vec![3, 4], vec![5, 8], vec![vec![1, 2]], 4), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_profit(3, vec![4, 6, 8], vec![7, 9, 11], vec![vec![1, 2], vec![1, 3]], 10),
            10
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            max_profit(3, vec![5, 2, 3], vec![8, 5, 6], vec![vec![1, 2], vec![2, 3]], 7),
            12
        );
    }
}
