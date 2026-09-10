/// LeetCode #3670 - Maximum Product of Two Integers With No Common Bits
fn max_product(nums: Vec<i32>) -> i64 {
    let mx = *nums.iter().max().unwrap();
    let l = 32 - mx.leading_zeros() as usize;
    let size = 1usize << l;
    let mut dp = vec![0i32; size];
    for &x in &nums {
        dp[x as usize] = x;
    }
    for i in 0..l {
        for mask in 0..size {
            if mask & (1 << i) == 0 {
                let nxt = mask | (1 << i);
                if dp[mask] > dp[nxt] {
                    dp[nxt] = dp[mask];
                }
            }
        }
    }
    let full = (size as i32) - 1;
    let mut result = 0i64;
    for &x in &nums {
        let v = x as i64 * dp[(full ^ x) as usize] as i64;
        if v > result {
            result = v;
        }
    }
    result
}

fn main() {
    println!("{}", max_product(vec![1, 2, 3, 4, 5, 6, 7]));
}

#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn example1() {
        assert_eq!(max_product(vec![1, 2, 3, 4, 5, 6, 7]), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(max_product(vec![5, 6, 4]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(max_product(vec![64, 8, 32]), 2048);
    }
}
