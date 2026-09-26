/// LeetCode #3958 - Minimum Cost to Split into Ones II
fn min_cost(n: i32) -> i64 {
    let n = n as i64;
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
