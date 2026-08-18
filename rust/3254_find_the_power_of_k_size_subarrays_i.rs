/// LeetCode #3254 - Find the Power of K-Size Subarrays I
fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut f = vec![1; n];
    for i in 1..n {
        if nums[i] == nums[i - 1] + 1 {
            f[i] = f[i - 1] + 1;
        }
    }
    (k - 1..n)
        .map(|i| if f[i] >= k { nums[i] } else { -1 })
        .collect()
}

fn main() {
    println!("{:?}", results_array(vec![1, 2, 3, 4, 3, 2, 5], 3));
}

#[cfg(test)]
mod tests {
    use super::results_array;

    #[test]
    fn example1() {
        assert_eq!(
            results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
            vec![3, 4, -1, -1, -1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(results_array(vec![2, 2, 2, 2, 2], 4), vec![-1, -1]);
    }

    #[test]
    fn example3() {
        assert_eq!(
            results_array(vec![3, 2, 3, 2, 3, 2], 2),
            vec![-1, 3, -1, 3, -1]
        );
    }
}
