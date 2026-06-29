/// LeetCode #2176 - Count Equal and Divisible Pairs in an Array
fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0i32;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] && (i * j) as i32 % k == 0 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::count_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_pairs(vec![1, 2, 3, 4], 1), 0);
    }
}
