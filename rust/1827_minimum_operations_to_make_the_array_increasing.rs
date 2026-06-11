/// LeetCode #1827 - Minimum Operations to Make the Array Increasing
fn min_operations(nums: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let mut mx = 0i32;
    for v in nums {
        ans += (mx + 1 - v).max(0);
        mx = (mx + 1).max(v);
    }
    ans
}

fn main() {
    println!("{}", min_operations(vec![1, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_operations(vec![1, 1, 1]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![1, 5, 2, 4, 1]), 14);
    }
}
