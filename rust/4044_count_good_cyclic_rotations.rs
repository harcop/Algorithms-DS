/// LeetCode #4044 - Count Good Cyclic Rotations
fn count_good_rotations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = n / 2;
    let mut left = 0i64;
    let mut right = 0i64;
    for i in 0..m {
        left += nums[i] as i64;
    }
    for i in m..n {
        right += nums[i] as i64;
    }
    let mut ans = i32::from(left > right);
    for i in 0..n - 1 {
        left -= nums[i] as i64;
        right += nums[i] as i64;
        let mid = nums[(i + m) % n] as i64;
        left += mid;
        right -= mid;
        if left > right {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_good_rotations(vec![1, 2, 3, 4, 5, 6]));
}

#[cfg(test)]
mod tests {
    use super::count_good_rotations;

    #[test]
    fn example1() {
        assert_eq!(count_good_rotations(vec![1, 2, 3, 4, 5, 6]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_good_rotations(vec![1, 2, 1, 2]), 0);
    }
}
