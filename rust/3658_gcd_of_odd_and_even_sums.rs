/// LeetCode #3658 - GCD of Odd and Even Sums
fn gcd_of_odd_even_sums(n: i32) -> i32 {
    n
}

fn main() {
    println!("{}", gcd_of_odd_even_sums(4));
}

#[cfg(test)]
mod tests {
    use super::gcd_of_odd_even_sums;

    #[test]
    fn example1() {
        assert_eq!(gcd_of_odd_even_sums(4), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(gcd_of_odd_even_sums(5), 5);
    }
}
