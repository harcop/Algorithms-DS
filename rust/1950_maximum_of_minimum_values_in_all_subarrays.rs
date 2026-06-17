/// LeetCode #1950 - Maximum of Minimum Values in All Subarrays
fn find_maximums(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut left = vec![-1i32; n];
    let mut right = vec![n as i32; n];
    let mut stk: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&j) = stk.last() {
            if nums[j] >= nums[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&j) = stk.last() {
            left[i] = j as i32;
        }
        stk.push(i);
    }

    stk.clear();
    for i in (0..n).rev() {
        while let Some(&j) = stk.last() {
            if nums[j] >= nums[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&j) = stk.last() {
            right[i] = j as i32;
        }
        stk.push(i);
    }

    let mut ans = vec![0i32; n];
    for i in 0..n {
        let m = (right[i] - left[i] - 1) as usize;
        ans[m - 1] = ans[m - 1].max(nums[i]);
    }
    for i in (0..n - 1).rev() {
        ans[i] = ans[i].max(ans[i + 1]);
    }
    ans
}

fn main() {
    println!("{:?}", find_maximums(vec![0, 1, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::find_maximums;

    #[test]
    fn example_one() {
        assert_eq!(find_maximums(vec![0, 1, 2, 4]), vec![4, 2, 1, 0]);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_maximums(vec![10, 20, 50, 10]), vec![50, 20, 10, 10]);
    }
}
