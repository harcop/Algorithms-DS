/// LeetCode #3730 - Maximum Calories Burnt from Jumps (premium)
fn max_calories_burnt(mut heights: Vec<i32>) -> i64 {
    heights.sort_unstable();
    let mut ans = 0i64;
    let mut pre = 0i32;
    let mut l = 0usize;
    let mut r = heights.len() - 1;
    while l < r {
        ans += (heights[r] - pre) as i64 * (heights[r] - pre) as i64;
        ans += (heights[l] - heights[r]) as i64 * (heights[l] - heights[r]) as i64;
        pre = heights[l];
        l += 1;
        r -= 1;
    }
    ans += (heights[r] - pre) as i64 * (heights[r] - pre) as i64;
    ans
}

fn main() {
    println!("{}", max_calories_burnt(vec![1, 7, 9]));
}

#[cfg(test)]
mod tests {
    use super::max_calories_burnt;

    #[test]
    fn example1() {
        assert_eq!(max_calories_burnt(vec![1, 7, 9]), 181);
    }

    #[test]
    fn example2() {
        assert_eq!(max_calories_burnt(vec![5, 2, 4]), 38);
    }

    #[test]
    fn example3() {
        assert_eq!(max_calories_burnt(vec![3, 3]), 9);
    }
}
