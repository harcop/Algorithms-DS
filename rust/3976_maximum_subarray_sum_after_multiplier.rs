/// LeetCode #3976 - Maximum Subarray Sum After Multiplier
fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as i64;
    let neg = i64::MIN / 4;
    let mut prev = [neg; 4];
    prev[0] = 0;
    let mut ans = neg;
    for x in nums {
        let x = x as i64;
        let cur = [
            prev[0].max(0) + x,
            prev[0].max(prev[1]).max(0) + x * k,
            prev[0].max(prev[2]).max(0) + x / k,
            prev[1].max(prev[2]).max(prev[3]) + x,
        ];
        ans = ans.max(cur[0]).max(cur[1]).max(cur[2]).max(cur[3]);
        prev = cur;
    }
    ans
}

fn main() {
    println!("{}", max_subarray_sum(vec![1, -2, 3, 4, -5], 2));
}

#[cfg(test)]
mod tests {
    use super::max_subarray_sum;

    #[test]
    fn example1() {
        assert_eq!(max_subarray_sum(vec![1, -2, 3, 4, -5], 2), 14);
    }

    #[test]
    fn example2() {
        assert_eq!(max_subarray_sum(vec![-5, -4, -3], 2), -1);
    }
}
