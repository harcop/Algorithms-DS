/// LeetCode #2413 - Smallest Even Multiple
fn smallest_even_multiple(n: i32) -> i32 {
    if n % 2 == 0 { n } else { n * 2 }
}

fn main() {
    println!("{}", smallest_even_multiple(5));
}

#[cfg(test)]
mod tests {
    use super::smallest_even_multiple;

    #[test]
    fn example_one() {
        assert_eq!(smallest_even_multiple(5), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_even_multiple(6), 6);
    }
}
