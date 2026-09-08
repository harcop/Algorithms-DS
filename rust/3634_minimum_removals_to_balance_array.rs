/// LeetCode #3634 - Minimum Removals to Balance Array
fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut ans = n;
    let mut r = 0usize;
    let k = k as i64;
    for l in 0..n {
        while r < n && nums[r] as i64 <= nums[l] as i64 * k {
            r += 1;
        }
        ans = ans.min(n - (r - l));
    }
    ans as i32
}

fn main() {
    println!("{}", min_removal(vec![2, 1, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::min_removal;

    #[test]
    fn example1() {
        assert_eq!(min_removal(vec![2, 1, 5], 2), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_removal(vec![1, 6, 2, 9], 3), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_removal(vec![4, 6], 2), 0);
    }
}
