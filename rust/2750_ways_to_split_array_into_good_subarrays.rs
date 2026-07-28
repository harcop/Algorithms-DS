/// LeetCode #2750 - Ways to Split Array Into Good Subarrays
fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ans = 1i64;
    let mut j = -1i64;
    for (i, &x) in nums.iter().enumerate() {
        if x == 0 {
            continue;
        }
        if j > -1 {
            ans = ans * (i as i64 - j) % MOD;
        }
        j = i as i64;
    }
    if j == -1 {
        0
    } else {
        ans as i32
    }
}

fn main() {
    println!("{}", number_of_good_subarray_splits(vec![0, 1, 0, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::number_of_good_subarray_splits;

    #[test]
    fn example_one() {
        assert_eq!(number_of_good_subarray_splits(vec![0, 1, 0, 0, 1]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_good_subarray_splits(vec![0, 1, 0]), 1);
    }
}
