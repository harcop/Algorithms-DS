/// LeetCode #983 - Minimum Cost For Tickets
fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let last = *days.last().unwrap() as usize;
    let mut dp = vec![0i32; last + 1];
    let mut idx = 0usize;
    for d in 1..=last {
        if idx < days.len() && days[idx] as usize == d {
            idx += 1;
            dp[d] = dp[d - 1] + costs[0];
            if d >= 7 {
                dp[d] = dp[d].min(dp[d - 7] + costs[1]);
            } else {
                dp[d] = dp[d].min(costs[1]);
            }
            if d >= 30 {
                dp[d] = dp[d].min(dp[d - 30] + costs[2]);
            } else {
                dp[d] = dp[d].min(costs[2]);
            }
        } else {
            dp[d] = dp[d - 1];
        }
    }
    dp[last]
}

fn main() {
    println!(
        "{}",
        mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15])
    );
}

#[cfg(test)]
mod tests {
    use super::mincost_tickets;

    #[test]
    fn example_one() {
        assert_eq!(
            mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]),
            11
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![2, 7, 15]),
            11
        );
    }
}
