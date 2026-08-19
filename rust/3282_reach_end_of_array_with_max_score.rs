/// LeetCode #3282 - Reach End of Array With Max Score
fn find_maximum_score(nums: Vec<i32>) -> i64 {
    let mut ans = 0i64;
    let mut mx = 0i64;
    for &x in &nums[..nums.len() - 1] {
        mx = mx.max(x as i64);
        ans += mx;
    }
    ans
}

fn main() {
    println!("{}", find_maximum_score(vec![1, 3, 1, 5]));
}

#[cfg(test)]
mod tests {
    use super::find_maximum_score;

    #[test]
    fn example1() {
        assert_eq!(find_maximum_score(vec![1, 3, 1, 5]), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(find_maximum_score(vec![4, 3, 1, 3, 2]), 16);
    }
}
