/// LeetCode #3993 - Maximum Value of an Alternating Sequence
fn maximum_value(n: i32, s: i32, m: i32) -> i64 {
    if n == 1 {
        return s as i64;
    }
    s as i64 + (n as i64 / 2) * (m as i64 - 1) + 1
}

fn main() {
    println!("{}", maximum_value(4, 3, 5));
}

#[cfg(test)]
mod tests {
    use super::maximum_value;

    #[test]
    fn example1() {
        assert_eq!(maximum_value(4, 3, 5), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_value(2, 4, 3), 7);
    }

    #[test]
    fn single() {
        assert_eq!(maximum_value(1, 9, 4), 9);
    }
}
