/// LeetCode #2436 - Minimum Split Into Subarrays With GCD Greater Than One
fn minimum_splits(nums: Vec<i32>) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }
        a
    }

    let mut answer = 1;
    let mut current_gcd = nums[0];

    for &num in &nums[1..] {
        let next_gcd = gcd(current_gcd, num);
        if next_gcd == 1 {
            answer += 1;
            current_gcd = num;
        } else {
            current_gcd = next_gcd;
        }
    }

    answer
}

fn main() {
    println!("{}", minimum_splits(vec![12, 6, 3, 14, 8]));
}

#[cfg(test)]
mod tests {
    use super::minimum_splits;

    #[test]
    fn requires_multiple_subarrays() {
        assert_eq!(minimum_splits(vec![12, 6, 3, 14, 8]), 2);
    }

    #[test]
    fn one_common_factor() {
        assert_eq!(minimum_splits(vec![2, 4, 8, 16]), 1);
    }
}
