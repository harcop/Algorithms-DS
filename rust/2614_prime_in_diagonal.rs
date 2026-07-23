/// LeetCode #2614 - Prime In Diagonal
fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
    fn is_prime(x: i32) -> bool {
        if x < 2 {
            return false;
        }
        let mut i = 2;
        while i <= x / i {
            if x % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    let n = nums.len();
    let mut ans = 0;
    for i in 0..n {
        if is_prime(nums[i][i]) {
            ans = ans.max(nums[i][i]);
        }
        if is_prime(nums[i][n - i - 1]) {
            ans = ans.max(nums[i][n - i - 1]);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        diagonal_prime(vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]])
    );
}

#[cfg(test)]
mod tests {
    use super::diagonal_prime;

    #[test]
    fn example_one() {
        assert_eq!(
            diagonal_prime(vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]]),
            11
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            diagonal_prime(vec![vec![1, 2, 3], vec![5, 17, 7], vec![9, 11, 10]]),
            17
        );
    }
}
