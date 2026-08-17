/// LeetCode #3251 - Find the Count of Monotonic Pairs II
fn count_of_pairs(nums: Vec<i32>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = nums.len();
    let m = *nums.iter().max().unwrap() as usize;
    let mut f = vec![vec![0i32; m + 1]; n];
    for j in 0..=nums[0] as usize {
        f[0][j] = 1;
    }
    for i in 1..n {
        let mut s = vec![0i32; m + 1];
        s[0] = f[i - 1][0];
        for j in 1..=m {
            s[j] = (s[j - 1] + f[i - 1][j]) % MOD;
        }
        for j in 0..=nums[i] as usize {
            let k = (j as i32).min(j as i32 + nums[i - 1] - nums[i]);
            if k >= 0 {
                f[i][j] = s[k as usize] % MOD;
            }
        }
    }
    f[n - 1][..=nums[n - 1] as usize]
        .iter()
        .fold(0, |a, &b| (a + b) % MOD)
}

fn main() {
    println!("{}", count_of_pairs(vec![2, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::count_of_pairs;

    #[test]
    fn example1() {
        assert_eq!(count_of_pairs(vec![2, 3, 2]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(count_of_pairs(vec![5, 5, 5, 5]), 126);
    }
}
