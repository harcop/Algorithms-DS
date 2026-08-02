/// LeetCode #2897 - Apply Operations on Array to Maximize Sum of Squares
fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut cnt = [0i32; 31];
    for x in nums {
        for i in 0..31 {
            if (x >> i) & 1 == 1 {
                cnt[i] += 1;
            }
        }
    }

    let mut ans = 0i64;
    for _ in 0..k {
        let mut x = 0i64;
        for i in 0..31 {
            if cnt[i] > 0 {
                x |= 1 << i;
                cnt[i] -= 1;
            }
        }
        ans = (ans + x * x) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", max_sum(vec![2, 6, 5, 8], 2));
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example_one() {
        assert_eq!(max_sum(vec![2, 6, 5, 8], 2), 261);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_sum(vec![4, 5, 4, 7], 3), 90);
    }
}
