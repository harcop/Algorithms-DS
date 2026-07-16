/// LeetCode #2420 - Find All Good Indices
fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut non_inc = vec![1; n];
    let mut non_dec = vec![1; n];

    for i in 1..n {
        if nums[i] <= nums[i - 1] {
            non_inc[i] = non_inc[i - 1] + 1;
        }
    }
    for i in (0..n - 1).rev() {
        if nums[i] <= nums[i + 1] {
            non_dec[i] = non_dec[i + 1] + 1;
        }
    }

    let mut ans = Vec::new();
    for i in k..n - k {
        if non_inc[i - 1] >= k && non_dec[i + 1] >= k {
            ans.push(i as i32);
        }
    }
    ans
}

fn main() {
    println!("{:?}", good_indices(vec![2, 1, 1, 1, 3, 4, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::good_indices;

    #[test]
    fn example_one() {
        assert_eq!(good_indices(vec![2, 1, 1, 1, 3, 4, 1], 2), vec![2, 3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(good_indices(vec![2, 1, 1, 2], 2), Vec::<i32>::new());
    }
}
