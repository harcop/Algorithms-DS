/// LeetCode #3101 - Count Alternating Subarrays
fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
    let mut ans = 1i64;
    let mut s = 1i64;
    for i in 1..nums.len() {
        s = if nums[i] != nums[i - 1] { s + 1 } else { 1 };
        ans += s;
    }
    ans
}

fn main() {
    println!("{}", count_alternating_subarrays(vec![0, 1, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::count_alternating_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_alternating_subarrays(vec![0, 1, 1, 1]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(count_alternating_subarrays(vec![1, 0, 1, 0]), 10);
    }
}
