/// LeetCode #3940 - Limit Occurrences in Sorted Array
fn limit_occurrences(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums;
    }
    let n = nums.len();
    let mut cnt = 1i32;
    let mut l = 1usize;
    for r in 1..n {
        if nums[r] != nums[r - 1] {
            cnt = 1;
        } else {
            cnt += 1;
        }
        if cnt <= k {
            nums[l] = nums[r];
            l += 1;
        }
    }
    nums.truncate(l);
    nums
}

fn main() {
    println!("{:?}", limit_occurrences(vec![1, 1, 1, 2, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::limit_occurrences;

    #[test]
    fn example1() {
        assert_eq!(
            limit_occurrences(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 1, 2, 2, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(limit_occurrences(vec![1, 2, 3], 1), vec![1, 2, 3]);
    }
}
