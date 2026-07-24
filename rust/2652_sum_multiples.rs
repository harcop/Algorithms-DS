/// LeetCode #2652 - Sum Multiples
fn sum_of_multiples(n: i32) -> i32 {
    (1..=n)
        .filter(|&x| x % 3 == 0 || x % 5 == 0 || x % 7 == 0)
        .sum()
}

fn main() {
    println!("{}", sum_of_multiples(7));
}

#[cfg(test)]
mod tests {
    use super::sum_of_multiples;

    #[test]
    fn example_one() {
        assert_eq!(sum_of_multiples(7), 21);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_of_multiples(10), 40);
    }

    #[test]
    fn example_three() {
        assert_eq!(sum_of_multiples(9), 30);
    }
}
