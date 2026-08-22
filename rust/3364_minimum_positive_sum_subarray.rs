/// LeetCode #3364 - Minimum Positive Sum Subarray
fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let n = nums.len();
    let l = l as usize;
    let r = r as usize;
    let mut ans = i32::MAX;
    for i in 0..n {
        let mut s = 0;
        for j in i..n {
            s += nums[j];
            let len = j - i + 1;
            if l <= len && len <= r && s > 0 {
                ans = ans.min(s);
            }
        }
    }
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3));
}

#[cfg(test)]
mod tests {
    use super::minimum_sum_subarray;

    #[test]
    fn example1() {
        assert_eq!(minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_sum_subarray(vec![-2, 2, -3, 1], 2, 3), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_sum_subarray(vec![1, 2, 3, 4], 2, 4), 3);
    }
}
