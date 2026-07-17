/// LeetCode #2447 - Number of Subarrays With GCD Equal to K
fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }
        a
    }

    let mut answer = 0;
    for start in 0..nums.len() {
        let mut current = 0;
        for &num in &nums[start..] {
            current = gcd(current, num);
            if current == k {
                answer += 1;
            }
            if current < k || current % k != 0 {
                break;
            }
        }
    }

    answer
}

fn main() {
    println!("{}", subarray_gcd(vec![9, 3, 1, 2, 6, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::subarray_gcd;

    #[test]
    fn example_one() {
        assert_eq!(subarray_gcd(vec![9, 3, 1, 2, 6, 3], 3), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(subarray_gcd(vec![4], 7), 0);
    }
}
