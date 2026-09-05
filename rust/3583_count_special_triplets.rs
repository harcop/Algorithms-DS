/// LeetCode #3583 - Count Special Triplets
use std::collections::HashMap;

fn special_triplets(nums: Vec<i32>) -> i32 {
    let mut left: HashMap<i32, i64> = HashMap::new();
    let mut right: HashMap<i32, i64> = HashMap::new();
    for &x in &nums {
        *right.entry(x).or_insert(0) += 1;
    }
    const MOD: i64 = 1_000_000_007;
    let mut ans = 0i64;
    for &x in &nums {
        *right.get_mut(&x).unwrap() -= 1;
        let t = x.saturating_mul(2);
        let l = *left.get(&t).unwrap_or(&0);
        let r = *right.get(&t).unwrap_or(&0);
        ans = (ans + (l * r) % MOD) % MOD;
        *left.entry(x).or_insert(0) += 1;
    }
    ans as i32
}

fn main() {
    println!("{}", special_triplets(vec![6, 3, 6]));
}

#[cfg(test)]
mod tests {
    use super::special_triplets;

    #[test]
    fn example1() {
        assert_eq!(special_triplets(vec![6, 3, 6]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(special_triplets(vec![0, 1, 0, 0]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(special_triplets(vec![8, 4, 2, 8, 4]), 2);
    }
}
