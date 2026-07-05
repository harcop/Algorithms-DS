/// LeetCode #2239 - Find Closest Number to Zero
fn find_closest_number(nums: Vec<i32>) -> i32 {
    let mut ans = nums[0];
    let mut best = ans.abs();
    for x in nums {
        let d = x.abs();
        if d < best || (d == best && x > ans) {
            ans = x;
            best = d;
        }
    }
    ans
}

fn main() {
    println!("{}", find_closest_number(vec![-4, -2, 1, 4, 8]));
}

#[cfg(test)]
mod tests {
    use super::find_closest_number;

    #[test]
    fn example_one() {
        assert_eq!(find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_closest_number(vec![2, -1, 1]), 1);
    }
}
