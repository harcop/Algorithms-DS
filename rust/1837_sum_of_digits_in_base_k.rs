/// LeetCode #1837 - Sum of Digits in Base K
fn sum_base(mut n: i32, k: i32) -> i32 {
    let mut ans = 0;
    while n > 0 {
        ans += n % k;
        n /= k;
    }
    ans
}

fn main() {
    println!("{}", sum_base(34, 6));
}

#[cfg(test)]
mod tests {
    use super::sum_base;

    #[test]
    fn example_one() {
        assert_eq!(sum_base(34, 6), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_base(10, 10), 1);
    }
}
