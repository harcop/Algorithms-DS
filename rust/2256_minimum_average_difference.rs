/// LeetCode #2256 - Minimum Average Difference
fn minimum_average_difference(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    let mut min_diff = i32::MAX;
    let mut prefix: i64 = 0;
    let mut suffix: i64 = nums.iter().map(|&x| x as i64).sum();

    for (i, &num) in nums.iter().enumerate() {
        prefix += num as i64;
        suffix -= num as i64;
        let prefix_avg = (prefix / (i + 1) as i64) as i32;
        let suffix_avg = if i == n - 1 {
            0
        } else {
            (suffix / (n - 1 - i) as i64) as i32
        };
        let diff = (prefix_avg - suffix_avg).abs();
        if diff < min_diff {
            ans = i as i32;
            min_diff = diff;
        }
    }

    ans
}

fn main() {
    println!("{}", minimum_average_difference(vec![2, 5, 3, 9, 5, 3]));
}

#[cfg(test)]
mod tests {
    use super::minimum_average_difference;

    #[test]
    fn example_one() {
        assert_eq!(minimum_average_difference(vec![2, 5, 3, 9, 5, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_average_difference(vec![0]), 0);
    }
}
