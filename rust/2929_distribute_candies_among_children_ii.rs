/// LeetCode #2929 - Distribute Candies Among Children II
fn comb2(n: i64) -> i64 {
    if n < 2 {
        0
    } else {
        n * (n - 1) / 2
    }
}

fn distribute_candies(n: i32, limit: i32) -> i64 {
    let n = n as i64;
    let limit = limit as i64;
    if n > 3 * limit {
        return 0;
    }
    let mut ans = comb2(n + 2);
    if n > limit {
        ans -= 3 * comb2(n - limit + 1);
    }
    if n - 2 >= 2 * limit {
        ans += 3 * comb2(n - 2 * limit);
    }
    ans
}

fn main() {
    println!("{}", distribute_candies(5, 2));
}

#[cfg(test)]
mod tests {
    use super::distribute_candies;

    #[test]
    fn example_one() {
        assert_eq!(distribute_candies(5, 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(distribute_candies(3, 3), 10);
    }
}
