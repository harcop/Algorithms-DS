/// LeetCode #2741 - Special Permutations
fn special_perm(nums: Vec<i32>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = nums.len();
    let m = 1 << n;
    let mut f = vec![vec![0; n]; m];
    for i in 1..m {
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                let ii = i ^ (1 << j);
                if ii == 0 {
                    f[i][j] = 1;
                    continue;
                }
                for k in 0..n {
                    if nums[j] % nums[k] == 0 || nums[k] % nums[j] == 0 {
                        f[i][j] = (f[i][j] + f[ii][k]) % MOD;
                    }
                }
            }
        }
    }
    f[m - 1].iter().fold(0, |acc, &x| (acc + x) % MOD)
}

fn main() {
    println!("{}", special_perm(vec![2, 3, 6]));
}

#[cfg(test)]
mod tests {
    use super::special_perm;

    #[test]
    fn example_one() {
        assert_eq!(special_perm(vec![2, 3, 6]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(special_perm(vec![1, 4, 3]), 2);
    }
}
