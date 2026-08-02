/// LeetCode #2932 - Maximum Strong Pair XOR I
fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for &x in &nums {
        for &y in &nums {
            if (x - y).abs() <= x.min(y) {
                ans = ans.max(x ^ y);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::maximum_strong_pair_xor;

    #[test]
    fn example_one() {
        assert_eq!(maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_strong_pair_xor(vec![10, 100]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_strong_pair_xor(vec![5, 6, 25, 30]), 7);
    }
}
