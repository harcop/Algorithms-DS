/// LeetCode #2188 - Minimum Time to Finish the Race
fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
    const INF: i32 = i32::MAX / 4;
    let change_time = change_time;
    let num_laps = num_laps as usize;

    let mut best_single = vec![INF; 18];
    for tire in &tires {
        let f = tire[0];
        let r = tire[1];
        let mut total = 0i32;
        let mut lap = 1usize;
        let mut t = f;
        while t <= change_time + f && lap < 18 {
            total += t;
            best_single[lap] = best_single[lap].min(total);
            t *= r;
            lap += 1;
        }
    }

    let mut dp = vec![INF; num_laps + 1];
    dp[0] = -change_time;
    for i in 1..=num_laps {
        for j in 1..=17.min(i) {
            if best_single[j] < INF {
                dp[i] = dp[i].min(dp[i - j] + best_single[j]);
            }
        }
        dp[i] += change_time;
    }

    dp[num_laps]
}

fn main() {
    println!(
        "{}",
        minimum_finish_time(vec![vec![2, 3], vec![3, 4]], 5, 4)
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_finish_time;

    #[test]
    fn example_one() {
        assert_eq!(minimum_finish_time(vec![vec![2, 3], vec![3, 4]], 5, 4), 21);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_finish_time(vec![vec![1, 10], vec![2, 2], vec![3, 4]], 6, 5),
            25
        );
    }
}
