/// LeetCode #656 - Coin Path
fn cheapest_jump(coins: Vec<i32>, max_jump: i32) -> Vec<i32> {
    let n = coins.len();
    if n == 0 || coins[0] < 0 || coins[n - 1] < 0 {
        return vec![];
    }
    let max_jump = max_jump as usize;
    const INF: i64 = 1_000_000_000_000;
    let mut dp = vec![INF; n];
    let mut nxt = vec![None; n];
    dp[n - 1] = coins[n - 1] as i64;
    for i in (0..n - 1).rev() {
        if coins[i] < 0 {
            continue;
        }
        let end = (i + max_jump).min(n - 1);
        for j in i + 1..=end {
            if coins[j] < 0 || dp[j] >= INF {
                continue;
            }
            let cost = coins[i] as i64 + dp[j];
            if cost < dp[i] {
                dp[i] = cost;
                nxt[i] = Some(j);
            }
        }
    }
    if dp[0] >= INF {
        return vec![];
    }
    let mut path = vec![];
    let mut i = 0;
    loop {
        path.push((i + 1) as i32);
        match nxt[i] {
            Some(j) => i = j,
            None => break,
        }
    }
    path
}

fn main() {
    println!("{:?}", cheapest_jump(vec![1, 2, 4, -1, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::cheapest_jump;

    #[test]
    fn example_one() {
        assert_eq!(cheapest_jump(vec![1, 2, 4, -1, 2], 2), vec![1, 3, 5]);
    }

    #[test]
    fn example_two() {
        assert_eq!(cheapest_jump(vec![1, 2, 4, -1, 2], 1), Vec::<i32>::new());
    }
}
