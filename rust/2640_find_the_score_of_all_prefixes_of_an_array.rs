/// LeetCode #2640 - Find the Score of All Prefixes of an Array
fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
    let n = nums.len();
    let mut ans = vec![0i64; n];
    let mut mx = 0i32;
    for i in 0..n {
        mx = mx.max(nums[i]);
        ans[i] = nums[i] as i64 + mx as i64;
        if i > 0 {
            ans[i] += ans[i - 1];
        }
    }
    ans
}

fn main() {
    println!("{:?}", find_prefix_score(vec![2, 3, 7, 5, 10]));
}

#[cfg(test)]
mod tests {
    use super::find_prefix_score;

    #[test]
    fn example_one() {
        assert_eq!(
            find_prefix_score(vec![2, 3, 7, 5, 10]),
            vec![4, 10, 24, 36, 56]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_prefix_score(vec![1, 1, 2, 4, 8, 16]),
            vec![2, 4, 8, 16, 32, 64]
        );
    }
}
