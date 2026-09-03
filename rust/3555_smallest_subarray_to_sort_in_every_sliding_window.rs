/// LeetCode #3555 - Smallest Subarray to Sort in Every Sliding Window
fn min_subarray_sort(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let f = |i: usize, j: usize| -> i32 {
        let mut mi = i32::MAX;
        let mut mx = i32::MIN;
        let mut l = -1i32;
        let mut r = -1i32;
        for p in i..=j {
            if nums[p] < mx {
                r = p as i32;
            } else {
                mx = nums[p];
            }
            let q = j - p + i;
            if nums[q] > mi {
                l = q as i32;
            } else {
                mi = nums[q];
            }
        }
        if r == -1 {
            0
        } else {
            r - l + 1
        }
    };
    (0..=n - k).map(|i| f(i, i + k - 1)).collect()
}

fn main() {
    println!("{:?}", min_subarray_sort(vec![1, 3, 2, 4, 5], 3));
}

#[cfg(test)]
mod tests {
    use super::min_subarray_sort;

    #[test]
    fn example1() {
        assert_eq!(min_subarray_sort(vec![1, 3, 2, 4, 5], 3), vec![2, 2, 0]);
    }

    #[test]
    fn example2() {
        assert_eq!(min_subarray_sort(vec![5, 4, 3, 2, 1], 4), vec![4, 4]);
    }
}
