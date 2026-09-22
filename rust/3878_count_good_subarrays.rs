/// LeetCode #3878 - Count Good Subarrays
fn count_good_subarrays(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut l = vec![-1i32; n];
    let mut stk: Vec<usize> = Vec::new();
    for i in 0..n {
        let x = nums[i];
        while stk
            .last()
            .map(|&j| nums[j] < x && (nums[j] | x) == x)
            .unwrap_or(false)
        {
            stk.pop();
        }
        l[i] = stk.last().copied().map(|j| j as i32).unwrap_or(-1);
        stk.push(i);
    }
    let mut r = vec![n as i32; n];
    stk.clear();
    for i in (0..n).rev() {
        while stk
            .last()
            .map(|&j| (nums[j] | nums[i]) == nums[i])
            .unwrap_or(false)
        {
            stk.pop();
        }
        r[i] = stk.last().copied().map(|j| j as i32).unwrap_or(n as i32);
        stk.push(i);
    }
    let mut ans = 0i64;
    for i in 0..n {
        ans += (i as i32 - l[i]) as i64 * (r[i] - i as i32) as i64;
    }
    ans
}

fn main() {
    println!("{}", count_good_subarrays(vec![4, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::count_good_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_good_subarrays(vec![4, 2, 3]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(count_good_subarrays(vec![1, 3, 1]), 6);
    }
}
