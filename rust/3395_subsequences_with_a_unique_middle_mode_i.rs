/// LeetCode #3395 - Subsequences with a Unique Middle Mode I
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn nc2(n: i64) -> i64 {
    if n < 2 {
        0
    } else {
        n * (n - 1) / 2 % MOD
    }
}

fn get(m: &HashMap<i32, i64>, k: i32) -> i64 {
    *m.get(&k).unwrap_or(&0)
}

fn calc(
    a: i32,
    other1: i64,
    other2: i64,
    count1: &HashMap<i32, i64>,
    count2: &HashMap<i32, i64>,
) -> i64 {
    let mut res = other1 * nc2(other2) % MOD;
    for (&b, &b1) in count1 {
        if b == a {
            continue;
        }
        let b2 = get(count2, b);
        res = (res - b1 * nc2(b2) % MOD + MOD) % MOD;
        res = (res - b1 * b2 % MOD * (other2 - b2) % MOD + MOD) % MOD;
    }
    for (&b, &b2) in count2 {
        if b == a {
            continue;
        }
        let b1 = get(count1, b);
        res = (res - (other1 - b1) * nc2(b2) % MOD + MOD) % MOD;
    }
    res
}

fn subsequences_with_middle_mode(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut left: HashMap<i32, i64> = HashMap::new();
    let mut right: HashMap<i32, i64> = HashMap::new();
    for &x in &nums[..2] {
        *left.entry(x).or_insert(0) += 1;
    }
    for &x in &nums[2..] {
        *right.entry(x).or_insert(0) += 1;
    }
    let mut ans = 0i64;
    for i in 2..n - 2 {
        let num = nums[i];
        let next = get(&right, num) - 1;
        if next <= 0 {
            right.remove(&num);
        } else {
            right.insert(num, next);
        }
        let left_count = get(&left, num);
        let right_count = get(&right, num);
        let left_other = i as i64 - left_count;
        let right_other = (n - 1 - i) as i64 - right_count;
        ans += nc2(left_count) * nc2(right_count);
        ans += nc2(left_count) * right_count % MOD * right_other % MOD;
        ans += left_count * left_other % MOD * nc2(right_count) % MOD;
        ans += nc2(left_count) * nc2(right_other) % MOD;
        ans += nc2(left_other) * nc2(right_count) % MOD;
        ans += left_count * left_other % MOD * right_count % MOD * right_other % MOD;
        ans += left_count * calc(num, left_other, right_other, &left, &right) % MOD;
        ans += right_count * calc(num, right_other, left_other, &right, &left) % MOD;
        ans %= MOD;
        *left.entry(num).or_insert(0) += 1;
    }
    ans as i32
}

fn main() {
    println!("{}", subsequences_with_middle_mode(vec![1, 1, 1, 1, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::subsequences_with_middle_mode;

    #[test]
    fn example1() {
        assert_eq!(subsequences_with_middle_mode(vec![1, 1, 1, 1, 1, 1]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(subsequences_with_middle_mode(vec![1, 2, 2, 3, 3, 4]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(
            subsequences_with_middle_mode(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            0
        );
    }
}
