/// LeetCode #1863 - Sum of All Subset XOR Totals
fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for mask in 0..(1 << n) {
        let mut s = 0;
        for j in 0..n {
            if mask >> j & 1 == 1 {
                s ^= nums[j];
            }
        }
        ans += s;
    }
    ans
}

fn main() {
    println!("{}", subset_xor_sum(vec![1, 3]));
}

#[cfg(test)]
mod tests {
    use super::subset_xor_sum;

    #[test]
    fn example_one() {
        assert_eq!(subset_xor_sum(vec![1, 3]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(subset_xor_sum(vec![5, 1, 6]), 28);
    }
}
