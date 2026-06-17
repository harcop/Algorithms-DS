/// LeetCode #1954 - Minimum Garden Perimeter to Collect Enough Apples
fn minimum_perimeter(needed_apples: i64) -> i64 {
    let mut x = 1i64;
    while 2 * x * (x + 1) * (2 * x + 1) < needed_apples {
        x += 1;
    }
    x * 8
}

fn main() {
    println!("{}", minimum_perimeter(1));
}

#[cfg(test)]
mod tests {
    use super::minimum_perimeter;

    #[test]
    fn example_one() {
        assert_eq!(minimum_perimeter(1), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_perimeter(13), 16);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_perimeter(1_000_000_000), 5040);
    }
}
