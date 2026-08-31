/// LeetCode #3502 - Minimum Cost to Reach Every Position
fn min_costs(cost: Vec<i32>) -> Vec<i32> {
    let mut mi = i32::MAX;
    cost.into_iter()
        .map(|c| {
            mi = mi.min(c);
            mi
        })
        .collect()
}

fn main() {
    println!("{:?}", min_costs(vec![5, 3, 4, 1, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_costs;

    #[test]
    fn example1() {
        assert_eq!(min_costs(vec![5, 3, 4, 1, 3, 2]), vec![5, 3, 3, 1, 1, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(min_costs(vec![1, 2, 4, 6, 7]), vec![1, 1, 1, 1, 1]);
    }
}
