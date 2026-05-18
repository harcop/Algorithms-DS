/// LeetCode #1063 - Number of Valid Subarrays
fn valid_subarrays(nums: Vec<i32>) -> i32 {
    let mut st: Vec<i32> = Vec::new();
    let mut ans = 0i32;
    for &x in &nums {
        while st.last().is_some_and(|&v| v > x) {
            st.pop();
        }
        st.push(x);
        ans += st.len() as i32;
    }
    ans
}

fn main() {
    println!("{}", valid_subarrays(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::valid_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(valid_subarrays(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(valid_subarrays(vec![3, 1, 2, 4]), 7);
    }
}
