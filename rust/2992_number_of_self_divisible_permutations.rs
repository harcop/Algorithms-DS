/// LeetCode #2992 - Number of Self-Divisible Permutations
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn self_divisible_permutation_count(n: i32) -> i32 {
    fn dfs(mask: i32, n: i32, memo: &mut [i32]) -> i32 {
        let i = mask.count_ones() as i32 + 1;
        if i > n {
            return 1;
        }
        if memo[mask as usize] != -1 {
            return memo[mask as usize];
        }
        let mut ans = 0;
        for j in 1..=n {
            if (mask >> j) & 1 == 0 && gcd(i, j) == 1 {
                ans += dfs(mask | (1 << j), n, memo);
            }
        }
        memo[mask as usize] = ans;
        ans
    }
    let mut memo = vec![-1; 1 << (n + 1)];
    dfs(0, n, &mut memo)
}

fn main() {
    println!("{}", self_divisible_permutation_count(3));
}

#[cfg(test)]
mod tests {
    use super::self_divisible_permutation_count;

    #[test]
    fn example_one() {
        assert_eq!(self_divisible_permutation_count(1), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(self_divisible_permutation_count(2), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(self_divisible_permutation_count(3), 3);
    }
}
