/// LeetCode #2894 - Divisible and Non-divisible Sums Difference
fn difference_of_sums(n: i32, m: i32) -> i32 {
    (1..=n).map(|i| if i % m == 0 { -i } else { i }).sum()
}

fn main() {
    println!("{}", difference_of_sums(10, 3));
}

#[cfg(test)]
mod tests {
    use super::difference_of_sums;

    #[test]
    fn example_one() {
        assert_eq!(difference_of_sums(10, 3), 19);
    }

    #[test]
    fn example_two() {
        assert_eq!(difference_of_sums(5, 6), 15);
    }

    #[test]
    fn example_three() {
        assert_eq!(difference_of_sums(5, 1), -15);
    }
}
