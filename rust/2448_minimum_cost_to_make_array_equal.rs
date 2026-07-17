/// LeetCode #2448 - Minimum Cost to Make Array Equal
fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
    let mut values: Vec<(i32, i64)> = nums
        .into_iter()
        .zip(cost)
        .map(|(num, weight)| (num, weight as i64))
        .collect();
    values.sort_unstable_by_key(|&(num, _)| num);

    let total_weight: i64 = values.iter().map(|&(_, weight)| weight).sum();
    let mut prefix_weight = 0;
    let mut target = values[0].0;
    for &(num, weight) in &values {
        prefix_weight += weight;
        if prefix_weight * 2 >= total_weight {
            target = num;
            break;
        }
    }

    values
        .into_iter()
        .map(|(num, weight)| (num as i64 - target as i64).abs() * weight)
        .sum()
}

fn main() {
    println!("{}", min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]), 8);
    }

    #[test]
    fn already_equal() {
        assert_eq!(min_cost(vec![2, 2, 2, 2, 2], vec![4, 2, 8, 1, 3]), 0);
    }
}
