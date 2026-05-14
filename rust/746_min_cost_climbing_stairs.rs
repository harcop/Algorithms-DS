/// LeetCode #746 - Min Cost Climbing Stairs
fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    if n == 1 {
        return cost[0];
    }
    let mut a = cost[0];
    let mut b = cost[1];
    for i in 2..n {
        let c = cost[i] + a.min(b);
        a = b;
        b = c;
    }
    a.min(b)
}

fn main() {
    println!("{}", min_cost_climbing_stairs(vec![10, 15, 20]));
}

#[cfg(test)]
mod tests {
    use super::min_cost_climbing_stairs;

    #[test]
    fn example_one() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]), 6);
    }
}
