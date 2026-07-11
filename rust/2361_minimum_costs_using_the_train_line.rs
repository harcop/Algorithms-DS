/// LeetCode #2361 - Minimum Costs Using the Train Line
fn minimum_costs(regular: Vec<i32>, express: Vec<i32>, express_cost: i32) -> Vec<i64> {
    let n = regular.len();
    let mut f = 0i64;
    let mut g = i64::MAX / 4;
    let mut cost = vec![0i64; n];
    for i in 0..n {
        let a = regular[i] as i64;
        let b = express[i] as i64;
        let ff = (f + a).min(g + a);
        let gg = (f + express_cost as i64 + b).min(g + b);
        f = ff;
        g = gg;
        cost[i] = f.min(g);
    }
    cost
}

fn main() {
    println!(
        "{:?}",
        minimum_costs(vec![1, 6, 9, 5], vec![5, 2, 3, 10], 8)
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_costs;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_costs(vec![1, 6, 9, 5], vec![5, 2, 3, 10], 8),
            vec![1, 7, 14, 19]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_costs(vec![11, 5, 13], vec![7, 10, 6], 3),
            vec![10, 15, 24]
        );
    }
}
