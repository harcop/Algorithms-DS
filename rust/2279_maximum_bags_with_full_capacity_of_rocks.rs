/// LeetCode #2279 - Maximum Bags With Full Capacity of Rocks
fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
    let mut diff: Vec<i32> = capacity
        .iter()
        .zip(rocks.iter())
        .map(|(&c, &r)| c - r)
        .collect();
    diff.sort_unstable();

    for (i, &d) in diff.iter().enumerate() {
        if d > additional_rocks {
            return i as i32;
        }
        additional_rocks -= d;
    }

    diff.len() as i32
}

fn main() {
    println!(
        "{}",
        maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_bags;

    #[test]
    fn example_one() {
        assert_eq!(maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100), 3);
    }
}
