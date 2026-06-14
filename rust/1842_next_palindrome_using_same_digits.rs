/// LeetCode #1842 - Next Palindrome Using Same Digits
fn next_palindrome(num: String) -> String {
    let mut nums: Vec<char> = num.chars().collect();
    let n = nums.len();
    let half = n / 2;

    fn next_permutation_left(nums: &mut [char]) -> bool {
        let m = nums.len();
        let mut i = m.saturating_sub(2);
        while nums[i] >= nums[i + 1] {
            if i == 0 {
                return false;
            }
            i -= 1;
        }
        let mut j = m - 1;
        while nums[j] <= nums[i] {
            j -= 1;
        }
        nums.swap(i, j);
        nums[i + 1..].reverse();
        true
    }

    if !next_permutation_left(&mut nums[..half]) {
        return String::new();
    }
    for i in 0..half {
        nums[n - 1 - i] = nums[i];
    }
    nums.iter().collect()
}

fn main() {
    println!("{}", next_palindrome("1221".into()));
}

#[cfg(test)]
mod tests {
    use super::next_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(next_palindrome("1221".into()), "2112");
    }

    #[test]
    fn example_two() {
        assert_eq!(next_palindrome("32123".into()), "");
    }

    #[test]
    fn example_three() {
        assert_eq!(next_palindrome("45544554".into()), "54455445");
    }
}
