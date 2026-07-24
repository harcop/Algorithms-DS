/// LeetCode #2653 - Sliding Subarray Beauty
fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let k = k as usize;
    let x = x as i32;
    let mut cnt = [0i32; 101];

    let f = |cnt: &[i32; 101], x: i32| -> i32 {
        let mut s = 0;
        for i in 0..50 {
            s += cnt[i];
            if s >= x {
                return i as i32 - 50;
            }
        }
        0
    };

    for &v in &nums[..k] {
        cnt[(v + 50) as usize] += 1;
    }
    let mut ans = vec![f(&cnt, x)];
    for i in k..nums.len() {
        cnt[(nums[i] + 50) as usize] += 1;
        cnt[(nums[i - k] + 50) as usize] -= 1;
        ans.push(f(&cnt, x));
    }
    ans
}

fn main() {
    println!("{:?}", get_subarray_beauty(vec![1, -1, -3, -2, 3], 3, 2));
}

#[cfg(test)]
mod tests {
    use super::get_subarray_beauty;

    #[test]
    fn example_one() {
        assert_eq!(
            get_subarray_beauty(vec![1, -1, -3, -2, 3], 3, 2),
            vec![-1, -2, -2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            get_subarray_beauty(vec![-1, -2, -3, -4, -5], 2, 2),
            vec![-1, -2, -3, -4]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            get_subarray_beauty(vec![-3, 1, 2, -3, 0, -3], 2, 1),
            vec![-3, 0, -3, -3, -3]
        );
    }
}
