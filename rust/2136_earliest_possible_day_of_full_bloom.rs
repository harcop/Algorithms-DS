/// LeetCode #2136 - Earliest Possible Day of Full Bloom
fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
    let mut seeds: Vec<(i32, i32)> = plant_time.into_iter().zip(grow_time).collect();
    seeds.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut planted = 0;
    let mut answer = 0;
    for (plant, grow) in seeds {
        planted += plant;
        answer = answer.max(planted + grow);
    }

    answer
}

fn main() {
    println!("{}", earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::earliest_full_bloom;

    #[test]
    fn example_one() {
        assert_eq!(earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            earliest_full_bloom(vec![1, 2, 3, 2], vec![2, 1, 2, 1]),
            9
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(earliest_full_bloom(vec![1], vec![1]), 2);
    }
}
