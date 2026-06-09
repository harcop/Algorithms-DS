/// LeetCode #1793 - Maximum Score of a Good Subarray
fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut left = vec![0i32; n];
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
        left[i] = stk.last().copied().map(|x| x as i32).unwrap_or(-1);
        stk.push(i);
    }
    stk.clear();
    for i in (0..n).rev() {
        while let Some(&j) = stk.last() {
            if nums[j] > nums[i] {
                stk.pop();
            } else {
                break;
            }
        }
        right[i] = stk.last().copied().map(|x| x as i32).unwrap_or(n as i32);
        stk.push(i);
    }
    let mut ans = 0i32;
    for i in 0..n {
        if left[i] + 1 <= k as i32 && (right[i] - 1) >= k as i32 {
            ans = ans.max(nums[i] * (right[i] - left[i] - 1));
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_score(vec![1, 4, 3, 7, 4, 5], 3));
}

#[cfg(test)]
mod tests {
    use super::maximum_score;

    #[test]
    fn example_one() {
        assert_eq!(maximum_score(vec![1, 4, 3, 7, 4, 5], 3), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0), 20);
    }
}
