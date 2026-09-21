/// LeetCode #3857 - Minimum Cost to Split into Ones
fn min_cost(n: i32) -> i32 {
    n * (n - 1) / 2
}

fn main() {
    println!("{}", min_cost(3));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cost(3), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost(4), 6);
    }
}
