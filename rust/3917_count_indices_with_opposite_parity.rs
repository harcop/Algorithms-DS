/// LeetCode #3917 - Count Indices With Opposite Parity
fn count_opposite_parity(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut odd_suf = vec![0i32; n + 1];
    let mut even_suf = vec![0i32; n + 1];
    for i in (0..n).rev() {
        odd_suf[i] = odd_suf[i + 1] + (nums[i] & 1);
        even_suf[i] = even_suf[i + 1] + ((nums[i] & 1) ^ 1);
    }
    (0..n)
        .map(|i| {
            if nums[i] & 1 == 1 {
                even_suf[i + 1]
            } else {
                odd_suf[i + 1]
            }
        })
        .collect()
}

fn main() {
    println!("{:?}", count_opposite_parity(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::count_opposite_parity;

    #[test]
    fn example1() {
        assert_eq!(count_opposite_parity(vec![1, 2, 3, 4]), vec![2, 1, 1, 0]);
    }

    #[test]
    fn example2() {
        assert_eq!(count_opposite_parity(vec![1]), vec![0]);
    }
}
