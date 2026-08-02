/// LeetCode #2917 - Find the K-or of an Array
fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    for i in 0..32 {
        let cnt = nums.iter().filter(|&&x| (x >> i) & 1 == 1).count() as i32;
        if cnt >= k {
            ans |= 1 << i;
        }
    }
    ans
}

fn main() {
    println!("{}", find_k_or(vec![7, 12, 9, 8, 9, 15], 4));
}

#[cfg(test)]
mod tests {
    use super::find_k_or;

    #[test]
    fn example_one() {
        assert_eq!(find_k_or(vec![7, 12, 9, 8, 9, 15], 4), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_k_or(vec![2, 12, 1, 11, 4, 5], 6), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_k_or(vec![10, 8, 5, 9, 11, 6, 8], 1), 15);
    }
}
