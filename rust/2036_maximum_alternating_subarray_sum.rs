/// LeetCode #2036 - Maximum Alternating Subarray Sum
fn maximum_alternating_subarray_sum(nums: Vec<i32>) -> i64 {
    let mut f = nums[0] as i64;
    let mut g = -f;
    let mut ans = f;
    for &x in &nums[1..] {
        let x = x as i64;
        let nf = g.max(0) + x;
        let ng = f - x;
        f = nf;
        g = ng;
        ans = ans.max(f).max(g);
    }
    ans
}

fn main() {
    println!("{}", maximum_alternating_subarray_sum(vec![3, -1, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::maximum_alternating_subarray_sum;

    #[test]
    fn example_one() {
        assert_eq!(maximum_alternating_subarray_sum(vec![3, -1, 1, 2]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_alternating_subarray_sum(vec![2, 2, 2, 2, 2]), 2);
    }
}
