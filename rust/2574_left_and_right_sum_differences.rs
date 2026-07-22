/// LeetCode #2574 - Left and Right Sum Differences
fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let mut l = 0;
    let mut r: i32 = nums.iter().sum();
    let mut ans = Vec::with_capacity(nums.len());
    for x in nums {
        r -= x;
        ans.push((l - r).abs());
        l += x;
    }
    ans
}

fn main() {
    println!("{:?}", left_right_difference(vec![10, 4, 8, 3]));
}

#[cfg(test)]
mod tests {
    use super::left_right_difference;

    #[test]
    fn example_one() {
        assert_eq!(left_right_difference(vec![10, 4, 8, 3]), vec![15, 1, 11, 22]);
    }

    #[test]
    fn example_two() {
        assert_eq!(left_right_difference(vec![1]), vec![0]);
    }
}
