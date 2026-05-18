/// LeetCode #1029 - Two City Scheduling
fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut diff: Vec<i32> = costs.iter().map(|c| c[0] - c[1]).collect();
    diff.sort_unstable();
    let n = costs.len() / 2;
    let mut sum: i32 = costs.iter().map(|c| c[1]).sum();
    for i in 0..n {
        sum += diff[i];
    }
    sum
}

fn main() {
    println!("{}", two_city_sched_cost(vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]]));
}

#[cfg(test)]
mod tests {
    use super::two_city_sched_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            two_city_sched_cost(vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]]),
            110
        );
    }
}
