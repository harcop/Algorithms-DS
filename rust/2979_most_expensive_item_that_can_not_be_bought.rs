/// LeetCode #2979 - Most Expensive Item That Can Not Be Bought
fn most_expensive_item(prime_one: i32, prime_two: i32) -> i32 {
    prime_one * prime_two - prime_one - prime_two
}

fn main() {
    println!("{}", most_expensive_item(2, 5));
}

#[cfg(test)]
mod tests {
    use super::most_expensive_item;

    #[test]
    fn example_one() {
        assert_eq!(most_expensive_item(2, 5), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(most_expensive_item(5, 7), 23);
    }
}
