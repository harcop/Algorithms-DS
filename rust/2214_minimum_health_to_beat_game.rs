/// LeetCode #2214 - Minimum Health to Beat Game
fn minimum_health(damage: Vec<i32>, armor: i32) -> i64 {
    let sum: i64 = damage.iter().map(|&x| x as i64).sum();
    let max_damage = *damage.iter().max().unwrap() as i64;
    1 + sum - max_damage.min(armor as i64)
}

fn main() {
    println!("{}", minimum_health(vec![2, 7, 10, 8], 4));
}

#[cfg(test)]
mod tests {
    use super::minimum_health;

    #[test]
    fn example_one() {
        assert_eq!(minimum_health(vec![2, 7, 10, 8], 4), 24);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_health(vec![3, 2, 4], 4), 6);
    }
}
