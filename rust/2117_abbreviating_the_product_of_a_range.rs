/// LeetCode #2117 - Abbreviating the Product of a Range
fn abbreviate_product(left: i32, right: i32) -> String {
    let mut suffix = 1i64;
    let mut count_two = 0i32;
    let mut count_five = 0i32;
    let mut log_sum = 0f64;
    const LIMIT: i64 = 10_000_000_000;

    for x in left..=right {
        log_sum += (x as f64).log10();

        let mut y = x;
        while y % 2 == 0 {
            count_two += 1;
            y /= 2;
        }
        while y % 5 == 0 {
            count_five += 1;
            y /= 5;
        }

        suffix = suffix * y as i64 % LIMIT;
    }

    let zeros = count_two.min(count_five);
    for _ in 0..count_two - zeros {
        suffix = suffix * 2 % LIMIT;
    }
    for _ in 0..count_five - zeros {
        suffix = suffix * 5 % LIMIT;
    }

    let remaining_log = log_sum - zeros as f64;
    let digits = remaining_log.floor() as i32 + 1;
    if digits > 10 {
        let leading = 10f64.powf(remaining_log.fract() + 4.0) as i64;
        format!("{}...{:05}e{}", leading, suffix % 100_000, zeros)
    } else {
        format!("{}e{}", suffix, zeros)
    }
}

fn main() {
    println!("{}", abbreviate_product(1, 4));
}

#[cfg(test)]
mod tests {
    use super::abbreviate_product;

    #[test]
    fn example_one() {
        assert_eq!(abbreviate_product(1, 4), "24e0");
    }

    #[test]
    fn example_two() {
        assert_eq!(abbreviate_product(2, 11), "399168e2");
    }

    #[test]
    fn example_three() {
        assert_eq!(abbreviate_product(371, 375), "7219856259e3");
    }

    #[test]
    fn abbreviates_large_product() {
        assert_eq!(abbreviate_product(1, 18), "64023...05728e3");
    }
}
