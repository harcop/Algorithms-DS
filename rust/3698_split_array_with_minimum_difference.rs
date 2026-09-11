/// LeetCode #3698 - Split Array With Minimum Difference
fn split_array(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut s = vec![0i64; n];
    s[0] = nums[0] as i64;
    for i in 1..n {
        s[i] = s[i - 1] + nums[i] as i64;
    }
    let mut f = vec![true; n];
    for i in 1..n {
        f[i] = f[i - 1] && nums[i] > nums[i - 1];
    }
    let mut g = vec![true; n];
    for i in (0..n - 1).rev() {
        g[i] = g[i + 1] && nums[i] > nums[i + 1];
    }
    let mut ans = i64::MAX;
    for i in 0..n - 1 {
        if f[i] && g[i + 1] {
            let s1 = s[i];
            let s2 = s[n - 1] - s[i];
            ans = ans.min((s1 - s2).abs());
        }
    }
    if ans == i64::MAX {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", split_array(vec![1, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::split_array;

    #[test]
    fn example1() {
        assert_eq!(split_array(vec![1, 3, 2]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(split_array(vec![1, 2, 4, 3]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(split_array(vec![3, 1, 2]), -1);
    }
}
