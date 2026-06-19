/// LeetCode #1979 - Find Greatest Common Divisor of Array
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn find_gcd(nums: Vec<i32>) -> i32 {
    let (mut lo, mut hi) = (i32::MAX, i32::MIN);
    for x in nums {
        lo = lo.min(x);
        hi = hi.max(x);
    }
    gcd(hi, lo)
}

fn main() {
    println!("{}", find_gcd(vec![2, 5, 6, 9, 10]));
}

#[cfg(test)]
mod tests {
    use super::find_gcd;

    #[test]
    fn example_one() {
        assert_eq!(find_gcd(vec![2, 5, 6, 9, 10]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_gcd(vec![7, 5, 6, 8, 3]), 1);
    }
}
