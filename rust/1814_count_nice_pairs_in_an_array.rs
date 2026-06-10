/// LeetCode #1814 - Count Nice Pairs in an Array
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn rev(mut x: i32) -> i32 {
    let mut y = 0i32;
    while x > 0 {
        y = y * 10 + x % 10;
        x /= 10;
    }
    y
}

fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i64> = HashMap::new();
    for x in nums {
        *cnt.entry(x - rev(x)).or_insert(0) += 1;
    }
    let mut ans = 0i64;
    for &v in cnt.values() {
        ans = (ans + v * (v - 1) / 2) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", count_nice_pairs(vec![42, 11, 1, 97]));
}

#[cfg(test)]
mod tests {
    use super::count_nice_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_nice_pairs(vec![42, 11, 1, 97]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
    }
}
