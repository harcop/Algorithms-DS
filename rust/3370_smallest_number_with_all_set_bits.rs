/// LeetCode #3370 - Smallest Number With All Set Bits
fn smallest_number(n: i32) -> i32 {
    let mut x = 1;
    while x - 1 < n {
        x <<= 1;
    }
    x - 1
}

fn main() {
    println!("{}", smallest_number(5));
}

#[cfg(test)]
mod tests {
    use super::smallest_number;

    #[test]
    fn example1() {
        assert_eq!(smallest_number(5), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(smallest_number(10), 15);
    }

    #[test]
    fn example3() {
        assert_eq!(smallest_number(3), 3);
    }
}
