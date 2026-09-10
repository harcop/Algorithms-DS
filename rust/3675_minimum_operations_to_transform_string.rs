/// LeetCode #3675 - Minimum Operations to Transform String
fn min_operations(s: String) -> i32 {
    let mut ans = 0;
    for c in s.bytes() {
        if c != b'a' {
            ans = ans.max(26 - (c - b'a') as i32);
        }
    }
    ans
}

fn main() {
    println!("{}", min_operations("yz".into()));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations("yz".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations("a".into()), 0);
    }
}
