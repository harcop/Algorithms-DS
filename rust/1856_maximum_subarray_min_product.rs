/// LeetCode #1856 - Maximum Subarray Min-Product
const MOD: i64 = 1_000_000_007;

fn max_sum_min_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut left = vec![-1isize; n];
    let mut right = vec![n as isize; n];
    let mut stk = Vec::new();

    for i in 0..n {
        while let Some(&top) = stk.last() {
            if nums[top] >= nums[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stk.last() {
            left[i] = top as isize;
        }
        stk.push(i);
    }

    stk.clear();
    for i in (0..n).rev() {
        while let Some(&top) = stk.last() {
            if nums[top] > nums[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stk.last() {
            right[i] = top as isize;
        }
        stk.push(i);
    }

    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + nums[i] as i64;
    }

    let mut best = 0i64;
    for i in 0..n {
        let sum = prefix[right[i] as usize] - prefix[(left[i] + 1) as usize];
        best = best.max(sum * nums[i] as i64);
    }
    (best % MOD) as i32
}

fn main() {
    println!("{}", max_sum_min_product(vec![1, 2, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::max_sum_min_product;

    #[test]
    fn example_one() {
        assert_eq!(max_sum_min_product(vec![1, 2, 3, 2]), 14);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_sum_min_product(vec![2, 3, 3, 1, 2]), 18);
    }
}
