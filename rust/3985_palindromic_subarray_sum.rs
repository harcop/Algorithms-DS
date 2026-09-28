/// LeetCode #3985 - Palindromic Subarray Sum
fn max_palindrome_sum(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut pref = vec![0i64; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + nums[i] as i64;
    }
    let mut ans = 0i64;
    let mut d1 = vec![0usize; n];
    let mut l: isize = 0;
    let mut r: isize = -1;
    for i in 0..n {
        let ii = i as isize;
        let mut k = if ii > r {
            1
        } else {
            (d1[(l + r - ii) as usize] as isize).min(r - ii + 1)
        };
        while ii - k >= 0
            && ii + k < n as isize
            && nums[(ii - k) as usize] == nums[(ii + k) as usize]
        {
            k += 1;
        }
        d1[i] = k as usize;
        let kk = k - 1;
        if ii + kk > r {
            l = ii - kk;
            r = ii + kk;
        }
        let rad = d1[i];
        ans = ans.max(pref[i + rad] - pref[i + 1 - rad]);
    }
    let mut d2 = vec![0usize; n];
    l = 0;
    r = -1;
    for i in 0..n {
        let ii = i as isize;
        let mut k = if ii > r {
            0
        } else {
            (d2[(l + r - ii + 1) as usize] as isize).min(r - ii + 1)
        };
        while ii - k - 1 >= 0
            && ii + k < n as isize
            && nums[(ii - k - 1) as usize] == nums[(ii + k) as usize]
        {
            k += 1;
        }
        d2[i] = k as usize;
        let kk = k - 1;
        if ii + kk > r {
            l = ii - kk - 1;
            r = ii + kk;
        }
        if d2[i] > 0 {
            ans = ans.max(pref[i + d2[i]] - pref[i - d2[i]]);
        }
    }
    ans
}

fn main() {
    println!("{}", max_palindrome_sum(vec![10, 10]));
}

#[cfg(test)]
mod tests {
    use super::max_palindrome_sum;

    #[test]
    fn example1() {
        assert_eq!(max_palindrome_sum(vec![10, 10]), 20);
    }

    #[test]
    fn example2() {
        assert_eq!(max_palindrome_sum(vec![1, 2, 3, 2, 1, 5, 6]), 9);
    }

    #[test]
    fn example3() {
        assert_eq!(max_palindrome_sum(vec![7, 1, 2, 1, 7, 3, 4, 3, 4]), 18);
    }

    #[test]
    fn example4() {
        assert_eq!(max_palindrome_sum(vec![1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn example5() {
        assert_eq!(max_palindrome_sum(vec![1000]), 1000);
    }
}
