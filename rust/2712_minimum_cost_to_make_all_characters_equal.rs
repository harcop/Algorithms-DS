/// LeetCode #2712 - Minimum Cost to Make All Characters Equal
fn minimum_cost(s: String) -> i64 {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut ans = 0i64;
    for i in 1..n {
        if bytes[i] != bytes[i - 1] {
            ans += i.min(n - i) as i64;
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_cost("0011".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(minimum_cost("0011".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_cost("010101".into()), 9);
    }
}
