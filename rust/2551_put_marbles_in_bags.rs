/// LeetCode #2551 - Put Marbles in Bags
fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    if k == 1 || weights.len() == 1 {
        return 0;
    }

    let mut arr: Vec<i64> = weights
        .windows(2)
        .map(|w| w[0] as i64 + w[1] as i64)
        .collect();
    arr.sort_unstable();

    let cuts = (k - 1) as usize;
    let mn: i64 = arr.iter().take(cuts).sum();
    let mx: i64 = arr.iter().rev().take(cuts).sum();
    mx - mn
}

fn main() {
    println!("{}", put_marbles(vec![1, 3, 5, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::put_marbles;

    #[test]
    fn example_one() {
        assert_eq!(put_marbles(vec![1, 3, 5, 1], 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(put_marbles(vec![1, 3], 2), 0);
    }
}
