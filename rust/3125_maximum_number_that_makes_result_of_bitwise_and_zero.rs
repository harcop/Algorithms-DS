/// LeetCode #3125 - Maximum Number That Makes Result of Bitwise AND Zero
fn max_number(n: i64) -> i64 {
    (1i64 << (63 - n.leading_zeros())) - 1
}

fn main() {
    println!("{}", max_number(7));
}

#[cfg(test)]
mod tests {
    use super::max_number;

    #[test]
    fn example1() {
        assert_eq!(max_number(7), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_number(9), 7);
    }

    #[test]
    fn example3() {
        assert_eq!(max_number(17), 15);
    }
}
