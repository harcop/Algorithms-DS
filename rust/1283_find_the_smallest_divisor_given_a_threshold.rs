/// LeetCode #1283 - Find the Smallest Divisor Given a Threshold
fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut lo = 1i32;
    let mut hi = *nums.iter().max().unwrap();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let sum: i64 = nums.iter().map(|&x| (x + mid - 1) / mid).sum::<i32>() as i64;
        if sum <= threshold as i64 {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    println!("{}", smallest_divisor(vec![1, 2, 5, 9], 6));
}

#[cfg(test)]
mod tests {
    use super::smallest_divisor;

    #[test]
    fn example_one() {
        assert_eq!(smallest_divisor(vec![1, 2, 5, 9], 6), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_divisor(vec![2, 3, 5, 7, 11], 11), 3);
    }
}
