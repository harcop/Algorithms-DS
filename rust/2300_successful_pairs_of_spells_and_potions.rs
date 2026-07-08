/// LeetCode #2300 - Successful Pairs of Spells and Potions
fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort_unstable();
    let m = potions.len();
    spells
        .into_iter()
        .map(|v| {
            let v = v as i64;
            let need = (success + v - 1) / v;
            let idx = potions.partition_point(|&p| (p as i64) < need);
            (m - idx) as i32
        })
        .collect()
}

fn main() {
    println!("{:?}", successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7));
}

#[cfg(test)]
mod tests {
    use super::successful_pairs;

    #[test]
    fn example_one() {
        assert_eq!(
            successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
            vec![2, 0, 2]
        );
    }
}
