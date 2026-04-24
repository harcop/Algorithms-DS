/// LeetCode #34 - Find First and Last Position of Element in Sorted Array
fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn lower_bound(nums: &[i32], target: i32) -> usize {
        let mut l = 0usize;
        let mut r = nums.len();
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }

    let left = lower_bound(&nums, target);
    if left == nums.len() || nums[left] != target {
        return vec![-1, -1];
    }
    let right = lower_bound(&nums, target + 1) - 1;
    vec![left as i32, right as i32]
}

fn main() {
    println!("{:?}", search_range(vec![5, 7, 7, 8, 8, 10], 8));
}

#[cfg(test)]
mod tests {
    use super::search_range;

    #[test]
    fn example_one() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    }

    #[test]
    fn example_three() {
        assert_eq!(search_range(vec![], 0), vec![-1, -1]);
    }
}
