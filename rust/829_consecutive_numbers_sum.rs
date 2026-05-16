/// LeetCode #829 - Consecutive Numbers Sum
fn consecutive_numbers_sum(n: i32) -> i32 {
    let mut count = 0;
    let mut k = 1;
    while k * (k - 1) / 2 < n {
        if (n - k * (k - 1) / 2) % k == 0 {
            count += 1;
        }
        k += 1;
    }
    count
}

fn main() {
    println!("{}", consecutive_numbers_sum(5));
}

#[cfg(test)]
mod tests {
    use super::consecutive_numbers_sum;

    #[test]
    fn example_one() {
        assert_eq!(consecutive_numbers_sum(5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(consecutive_numbers_sum(9), 3);
    }
}
