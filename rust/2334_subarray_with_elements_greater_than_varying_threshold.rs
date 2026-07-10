/// LeetCode #2334 - Subarray With Elements Greater Than Varying Threshold
fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
    let n = nums.len();
    let mut left = vec![-1i32; n];
    let mut right = vec![n as i32; n];
    let mut stk: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&top) = stk.last() {
            if nums[top] >= nums[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stk.last() {
            left[i] = top as i32;
        }
        stk.push(i);
    }

    stk.clear();
    for i in (0..n).rev() {
        while let Some(&top) = stk.last() {
            if nums[top] >= nums[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stk.last() {
            right[i] = top as i32;
        }
        stk.push(i);
    }

    for i in 0..n {
        let k = right[i] - left[i] - 1;
        if nums[i] as i64 > threshold as i64 / k as i64 {
            return k;
        }
    }
    -1
}

fn main() {
    println!("{}", valid_subarray_size(vec![1, 3, 4, 3, 1], 6));
}

#[cfg(test)]
mod tests {
    use super::valid_subarray_size;

    #[test]
    fn example_one() {
        assert_eq!(valid_subarray_size(vec![1, 3, 4, 3, 1], 6), 3);
    }

    #[test]
    fn example_two() {
        let nums = vec![6, 5, 6, 5, 8];
        let k = valid_subarray_size(nums.clone(), 7);
        assert!(k >= 1);
        let mut found = false;
        for start in 0..=nums.len() - k as usize {
            let sub = &nums[start..start + k as usize];
            let min = *sub.iter().min().unwrap();
            if min as i64 > 7 / k as i64 {
                found = true;
                break;
            }
        }
        assert!(found);
    }
}
