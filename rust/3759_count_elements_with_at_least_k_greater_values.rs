/// LeetCode #3759 - Count Elements With at Least K Greater Values
fn count_elements(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    if k == 0 {
        return n as i32;
    }
    nums.sort_unstable();
    let k = k as usize;
    let mut ans = 0;
    for i in 0..n - k {
        if nums[n - k] > nums[i] {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_elements(vec![3, 1, 2], 1));
}

#[cfg(test)]
mod tests {
    use super::count_elements;

    #[test]
    fn example1() {
        assert_eq!(count_elements(vec![3, 1, 2], 1), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_elements(vec![5, 5, 5], 2), 0);
    }
}
