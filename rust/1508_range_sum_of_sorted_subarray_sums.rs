/// LeetCode #1508 - Range Sum Of Sorted Subarray Sums
const MOD: i64 = 1_000_000_007;

fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    let mut sums: Vec<i64> = Vec::new();
    for i in 0..nums.len() {
        let mut s = 0i64;
        for j in i..nums.len() {
            s += nums[j] as i64;
            sums.push(s);
        }
    }
    sums.sort_unstable();
    let l = (left - 1) as usize;
    let r = right as usize;
    (sums[l..r].iter().sum::<i64>() % MOD) as i32
}

fn main() {
    println!("{}", range_sum(vec![1, 2, 3, 4], 4, 1, 5));
}

#[cfg(test)]
mod tests {
    use super::range_sum;

    #[test]
    fn example_one() {
        assert_eq!(range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
    }

    #[test]
    fn example_two() {
        assert_eq!(range_sum(vec![1, 2, 3, 4], 4, 3, 4), 6);
    }
}
