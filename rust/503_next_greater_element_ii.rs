/// LeetCode #503 - Next Greater Element II
fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![-1; n];
    let mut st = Vec::new();
    for i in 0..2 * n {
        let x = nums[i % n];
        while st.last().map(|&j| nums[j] < x).unwrap_or(false) {
            ans[st.pop().unwrap()] = x;
        }
        if i < n {
            st.push(i);
        }
    }
    ans
}

fn main() {
    println!("{:?}", next_greater_elements(vec![1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::next_greater_elements;

    #[test]
    fn example_one() {
        assert_eq!(next_greater_elements(vec![1, 2, 1]), vec![2, -1, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            next_greater_elements(vec![1, 2, 3, 4, 3]),
            vec![2, 3, 4, -1, 4]
        );
    }
}
