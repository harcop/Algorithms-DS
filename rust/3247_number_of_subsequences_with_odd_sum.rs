/// LeetCode #3247 - Number of Subsequences with Odd Sum
fn subsequence_count(nums: Vec<i32>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let mut f = [0i32; 2];
    for x in nums {
        if x % 2 == 1 {
            let (a, b) = (f[0], f[1]);
            f[0] = (a + b) % MOD;
            f[1] = (a + b + 1) % MOD;
        } else {
            let (a, b) = (f[0], f[1]);
            f[0] = (a + a + 1) % MOD;
            f[1] = (b + b) % MOD;
        }
    }
    f[1]
}

fn main() {
    println!("{}", subsequence_count(vec![1, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::subsequence_count;

    #[test]
    fn example1() {
        assert_eq!(subsequence_count(vec![1, 1, 1]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(subsequence_count(vec![1, 2, 2]), 4);
    }
}
