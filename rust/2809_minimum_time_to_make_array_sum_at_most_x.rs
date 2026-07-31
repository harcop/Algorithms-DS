/// LeetCode #2809 - Minimum Time to Make Array Sum At Most x
fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
    let n = nums1.len();
    let mut nums: Vec<(i32, i32)> = nums1.iter().copied().zip(nums2.iter().copied()).collect();
    nums.sort_by_key(|&(_, b)| b);
    let mut f = vec![0i32; n + 1];
    for &(a, b) in &nums {
        for j in (1..=n).rev() {
            f[j] = f[j].max(f[j - 1] + a + b * j as i32);
        }
    }
    let s1: i32 = nums1.iter().sum();
    let s2: i32 = nums2.iter().sum();
    for j in 0..=n {
        if s1 + s2 * j as i32 - f[j] <= x {
            return j as i32;
        }
    }
    -1
}

fn main() {
    println!("{}", minimum_time(vec![1, 2, 3], vec![1, 2, 3], 4));
}

#[cfg(test)]
mod tests {
    use super::minimum_time;

    #[test]
    fn example_one() {
        assert_eq!(minimum_time(vec![1, 2, 3], vec![1, 2, 3], 4), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_time(vec![1, 2, 3], vec![3, 3, 3], 4), -1);
    }
}
