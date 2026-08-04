/// LeetCode #2963 - Count the Number of Good Partitions
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut base: i64, mut exp: i32) -> i64 {
    let mut res = 1i64;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    res
}

fn number_of_good_partitions(nums: Vec<i32>) -> i32 {
    let mut last = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        last.insert(x, i);
    }
    let mut j = 0usize;
    let mut k = 0i32;
    for (i, &x) in nums.iter().enumerate() {
        j = j.max(*last.get(&x).unwrap());
        if i == j {
            k += 1;
        }
    }
    mod_pow(2, k - 1) as i32
}

fn main() {
    println!("{}", number_of_good_partitions(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::number_of_good_partitions;

    #[test]
    fn example_one() {
        assert_eq!(number_of_good_partitions(vec![1, 2, 3, 4]), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_good_partitions(vec![1, 1, 1, 1]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_good_partitions(vec![1, 2, 1, 3]), 2);
    }
}
