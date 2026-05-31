/// LeetCode #1569 - Number Of Ways To Reorder Array To Get Same Bst
const MOD: i64 = 1_000_000_007;

fn num_of_ways(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut fact = vec![1i64; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64 % MOD;
    }
    fn inv(a: i64) -> i64 {
        let mut t = 0i64;
        let mut newt = 1i64;
        let mut r = MOD;
        let mut newr = a;
        while newr != 0 {
            let q = r / newr;
            (t, newt) = (newt, t - q * newt);
            (r, newr) = (newr, r - q * newr);
        }
        if r > 1 { return 1; }
        if t < 0 { t += MOD; }
        t
    }
    fn comb(n: usize, k: usize, fact: &[i64]) -> i64 {
        if k > n { return 0; }
        fact[n] * inv(fact[k] * fact[n - k] % MOD) % MOD
    }
    fn dfs(nums: &[i32], fact: &[i64]) -> i64 {
        if nums.len() <= 2 { return 1; }
        let root = nums[0];
        let mut left = vec![];
        let mut right = vec![];
        for &x in &nums[1..] {
            if x < root { left.push(x); } else { right.push(x); }
        }
        let l = dfs(&left, fact);
        let r = dfs(&right, fact);
        comb(left.len() + right.len(), left.len(), fact) * l % MOD * r % MOD
    }
    ((dfs(&nums, &fact) - 1 + MOD) % MOD) as i32
}
fn main() { println!("{}", num_of_ways(vec![2,1,3])); }
#[cfg(test)]
mod tests {
    use super::num_of_ways;
    #[test]
    fn example_one() { assert_eq!(num_of_ways(vec![2,1,3]), 1); }
    #[test]
    fn example_two() { assert_eq!(num_of_ways(vec![3,1,2,5,4,6]), 19); }
}