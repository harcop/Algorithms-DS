/// LeetCode #2240 - Number of Ways to Buy Pens and Pencils
fn ways_to_buy_pens_pencils(total: i64, cost1: i64, cost2: i64) -> i64 {
    let mut ans = 0i64;
    let max_pens = total / cost1;
    for pens in 0..=max_pens {
        let remaining = total - pens * cost1;
        ans += remaining / cost2 + 1;
    }
    ans
}

fn main() {
    println!("{}", ways_to_buy_pens_pencils(20, 10, 5));
}

#[cfg(test)]
mod tests {
    use super::ways_to_buy_pens_pencils;

    #[test]
    fn example_one() {
        assert_eq!(ways_to_buy_pens_pencils(20, 10, 5), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(ways_to_buy_pens_pencils(5, 10, 10), 1);
    }
}
