/// LeetCode #1281 - Subtract the Product and Sum of Digits of an Integer
fn subtract_product_and_sum(n: i32) -> i32 {
    let mut n = n;
    let mut prod = 1;
    let mut sum = 0;
    while n > 0 {
        let d = n % 10;
        prod *= d;
        sum += d;
        n /= 10;
    }
    prod - sum
}

fn main() {
    println!("{}", subtract_product_and_sum(234));
}

#[cfg(test)]
mod tests {
    use super::subtract_product_and_sum;

    #[test]
    fn example_one() {
        assert_eq!(subtract_product_and_sum(234), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(subtract_product_and_sum(442), 2);
    }
}
