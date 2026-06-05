/// LeetCode #1774 - Closest Dessert Cost
fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
    let mut topping_sums = vec![0i32];
    fn dfs(i: usize, t: i32, topping_costs: &[i32], sums: &mut Vec<i32>) {
        if i >= topping_costs.len() {
            sums.push(t);
            return;
        }
        dfs(i + 1, t, topping_costs, sums);
        dfs(i + 1, t + topping_costs[i], topping_costs, sums);
        dfs(i + 1, t + 2 * topping_costs[i], topping_costs, sums);
    }
    dfs(0, 0, &topping_costs, &mut topping_sums);
    topping_sums.sort_unstable();
    let mut best = i32::MAX;
    let mut best_dist = i32::MAX;
    for b in base_costs {
        for &s in &topping_sums {
            let cost = b + s;
            let dist = (cost - target).abs();
            if dist < best_dist || (dist == best_dist && cost < best) {
                best_dist = dist;
                best = cost;
            }
        }
    }
    best
}
fn main() {
    println!(
        "{}",
        closest_cost(vec![1, 7], vec![3, 4], 10)
    );
}
#[cfg(test)]
mod tests {
    use super::closest_cost;
    #[test]
    fn example_one() {
        assert_eq!(closest_cost(vec![1, 7], vec![3, 4], 10), 10);
    }
    #[test]
    fn example_two() {
        assert_eq!(closest_cost(vec![3, 10], vec![2, 5], 9), 8);
    }
}
