/// LeetCode #3865 - Reverse K Subarrays (premium)
fn reverse_subarrays(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let m = n / k as usize;
    for i in (0..n).step_by(m) {
        nums[i..i + m].reverse();
    }
    nums
}

fn main() {
    println!("{:?}", reverse_subarrays(vec![1, 2, 4, 3, 5, 6], 3));
}

#[cfg(test)]
mod tests {
    use super::reverse_subarrays;

    #[test]
    fn example1() {
        assert_eq!(
            reverse_subarrays(vec![1, 2, 4, 3, 5, 6], 3),
            vec![2, 1, 3, 4, 6, 5]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(reverse_subarrays(vec![5, 4, 4, 2], 1), vec![2, 4, 4, 5]);
    }
}
