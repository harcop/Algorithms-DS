/// LeetCode #3684 - Maximize Sum of At Most K Distinct Elements
fn max_k_distinct(mut nums: Vec<i32>, mut k: i32) -> Vec<i32> {
    nums.sort_unstable();
    let n = nums.len();
    let mut ans = Vec::new();
    for i in (0..n).rev() {
        if i + 1 < n && nums[i] == nums[i + 1] {
            continue;
        }
        ans.push(nums[i]);
        k -= 1;
        if k == 0 {
            break;
        }
    }
    ans
}

fn main() {
    println!("{:?}", max_k_distinct(vec![84, 93, 100, 77, 90], 3));
}

#[cfg(test)]
mod tests {
    use super::max_k_distinct;

    #[test]
    fn example1() {
        assert_eq!(max_k_distinct(vec![84, 93, 100, 77, 90], 3), vec![100, 93, 90]);
    }

    #[test]
    fn example2() {
        assert_eq!(max_k_distinct(vec![84, 93, 100, 77, 93], 3), vec![100, 93, 84]);
    }

    #[test]
    fn example3() {
        assert_eq!(max_k_distinct(vec![1, 1, 1, 2, 2, 2], 6), vec![2, 1]);
    }
}
