/// LeetCode #3115 - Maximum Prime Difference
fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    let mut v = 2;
    while v * v <= x {
        if x % v == 0 {
            return false;
        }
        v += 1;
    }
    true
}

fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    loop {
        if is_prime(nums[i]) {
            let mut j = nums.len() - 1;
            loop {
                if is_prime(nums[j]) {
                    return (j - i) as i32;
                }
                j -= 1;
            }
        }
        i += 1;
    }
}

fn main() {
    println!("{}", maximum_prime_difference(vec![4, 2, 9, 5, 3]));
}

#[cfg(test)]
mod tests {
    use super::maximum_prime_difference;

    #[test]
    fn example1() {
        assert_eq!(maximum_prime_difference(vec![4, 2, 9, 5, 3]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_prime_difference(vec![4, 8, 2, 8]), 0);
    }
}
