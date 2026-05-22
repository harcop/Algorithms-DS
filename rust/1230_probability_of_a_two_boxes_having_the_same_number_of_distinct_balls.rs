/// LeetCode #1230 - Probability of a Two Boxes Having The Same Number of Distinct Balls
fn comb(n: i32, k: i32) -> i64 {
    if k < 0 || k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut res = 1i64;
    for i in 0..k {
        res = res * (n - i) as i64 / (i + 1) as i64;
    }
    res
}

fn probability_of_heads(balls: Vec<i32>) -> f64 {
    let total: i32 = balls.iter().sum();
    let half = total / 2;
    let mut ways_same = 0i64;
    let mut ways_total = 0i64;

    fn dfs(
        balls: &[i32],
        idx: usize,
        cnt1: i32,
        dist1: i32,
        dist2: i32,
        ways: i64,
        ways_same: &mut i64,
        ways_total: &mut i64,
        half: i32,
    ) {
        if idx == balls.len() {
            if cnt1 == half {
                *ways_total += ways;
                if dist1 == dist2 {
                    *ways_same += ways;
                }
            }
            return;
        }
        for take in 0..=balls[idx] {
            if cnt1 + take > half {
                continue;
            }
            let w = ways * comb(balls[idx], take);
            dfs(
                balls,
                idx + 1,
                cnt1 + take,
                dist1 + if take > 0 { 1 } else { 0 },
                dist2 + if balls[idx] - take > 0 { 1 } else { 0 },
                w,
                ways_same,
                ways_total,
                half,
            );
        }
    }

    dfs(&balls, 0, 0, 0, 0, 1, &mut ways_same, &mut ways_total, half);
    ways_same as f64 / ways_total as f64
}

fn main() {
    println!("{}", probability_of_heads(vec![1, 1]));
}

#[cfg(test)]
mod tests {
    use super::probability_of_heads;

    #[test]
    fn example_one() {
        assert!((probability_of_heads(vec![1, 1]) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn example_two() {
        assert!((probability_of_heads(vec![2, 1, 1]) - 0.66667).abs() < 1e-4);
    }

    #[test]
    fn example_three() {
        assert!((probability_of_heads(vec![1, 2, 1, 2]) - 0.6).abs() < 1e-4);
    }
}
