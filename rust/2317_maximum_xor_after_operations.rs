/// LeetCode #2317 - Maximum XOR After Operations
fn maximum_xor(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for x in nums {
        ans |= x;
    }
    ans
}

fn main() {
    println!("{}", maximum_xor(vec![3, 2, 4, 6]));
}

#[cfg(test)]
mod tests {
    use super::maximum_xor;

    #[test]
    fn example_one() {
        assert_eq!(maximum_xor(vec![3, 2, 4, 6]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_xor(vec![1, 2, 3, 9, 2]), 11);
    }
}
