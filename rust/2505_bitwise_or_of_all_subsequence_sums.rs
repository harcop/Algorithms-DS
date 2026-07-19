/// LeetCode #2505 - Bitwise OR of All Subsequence Sums
fn subsequence_sum_or(nums: Vec<i32>) -> i64 {
    let mut cnt = [0i64; 64];
    let mut ans = 0i64;
    for v in nums {
        for i in 0..31 {
            if (v >> i) & 1 == 1 {
                cnt[i] += 1;
            }
        }
    }
    for i in 0..63 {
        if cnt[i] > 0 {
            ans |= 1i64 << i;
        }
        cnt[i + 1] += cnt[i] / 2;
    }
    ans
}

fn main() {
    println!("{}", subsequence_sum_or(vec![2, 1, 0, 3]));
}

#[cfg(test)]
mod tests {
    use super::subsequence_sum_or;

    #[test]
    fn example_one() {
        assert_eq!(subsequence_sum_or(vec![2, 1, 0, 3]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(subsequence_sum_or(vec![0, 0, 0]), 0);
    }
}
