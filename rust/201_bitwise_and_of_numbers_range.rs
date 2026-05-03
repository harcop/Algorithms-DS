/// LeetCode #201 - Bitwise AND of Numbers Range
fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut l = left as u32;
    let mut r = right as u32;
    let mut s = 0u32;
    while l != r {
        l >>= 1;
        r >>= 1;
        s += 1;
    }
    (l << s) as i32
}

fn main() {
    println!("{}", range_bitwise_and(5, 7));
}

#[cfg(test)]
mod tests {
    use super::range_bitwise_and;

    #[test]
    fn example_one() {
        assert_eq!(range_bitwise_and(5, 7), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(range_bitwise_and(0, 0), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(range_bitwise_and(1, 2147483647), 0);
    }
}
