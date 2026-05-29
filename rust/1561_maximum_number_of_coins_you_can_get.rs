/// LeetCode #1561 - Maximum Number Of Coins You Can Get
fn max_coins(mut piles: Vec<i32>) -> i32 {
    piles.sort_unstable();
    let k = piles.len() / 3;
    piles[k - 1] + piles[k] + piles[piles.len() - k]
}

fn main() {
    println!("{}", max_coins(vec![2, 4, 5, 1, 5, 7, 8, 9, 9, 10, 11]));
}

#[cfg(test)]
mod tests {
    use super::max_coins;

    #[test]
    fn example_one() {
        assert_eq!(max_coins(vec![2, 4, 5, 1, 5, 7, 8, 9, 9, 10, 11]), 18);
    }
}
