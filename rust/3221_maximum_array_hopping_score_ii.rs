/// LeetCode #3221 - Maximum Array Hopping Score II
fn max_score(nums: Vec<i32>) -> i64 {
    let mut stk = Vec::new();
    for (i, &x) in nums.iter().enumerate() {
        while stk.last().is_some_and(|&j| nums[j] <= x) {
            stk.pop();
        }
        stk.push(i);
    }
    let mut ans = 0i64;
    let mut i = 0usize;
    for &j in &stk {
        ans += nums[j] as i64 * (j - i) as i64;
        i = j;
    }
    ans
}

fn main() {
    println!("{}", max_score(vec![1, 5, 8]));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(max_score(vec![1, 5, 8]), 16);
    }

    #[test]
    fn example2() {
        assert_eq!(max_score(vec![4, 5, 2, 8, 9, 1, 3]), 42);
    }
}
